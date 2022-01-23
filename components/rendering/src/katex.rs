use std::{str, vec};

use config::Delimiter;
use errors::{Error, Result};
use katex::{self, Opts};
use regex::Regex;

/// All the information from render HTML from katex
#[derive(Debug)]
pub struct KatexContext {
    enable: bool,
    restrict: bool,
    delimiters: Vec<Delimiter>,
}

impl KatexContext {
    pub fn new(enable: bool, restrict: bool, delimiters: Vec<Delimiter>) -> Self {
        KatexContext { enable, restrict, delimiters }
    }
}

#[derive(Debug)]
enum Segment {
    Text { data: String },
    Math { data: String, raw_data: String, display: bool },
    Unclosed(String),
}

fn find_end_of_math(delimiter: &str, text: &str, start_index: usize) -> Option<usize> {
    let mut brace_level = 0;
    let bytes = text.as_bytes();
    let mut index = start_index;
    while index < bytes.len() {
        let byte = bytes[index];
        let end = index + delimiter.len();
        if brace_level <= 0 && bytes.get(index..end)? == delimiter.as_bytes() {
            return Some(index);
        } else if byte == b'\\' {
            index += 1;
        } else if byte == b'{' {
            brace_level += 1;
        } else if byte == b'}' {
            brace_level -= 1;
        }
        index += 1;
    }
    None
}

fn escape_replace(text: &str) -> String {
    let re = Regex::new(r"[-/\\^$*+?.()|\[\]{}]").unwrap();
    re.replace_all(text, "\\$0").to_string()
}

fn spilt_at_delimiters(text: &str, delimiters: &Vec<Delimiter>) -> Vec<Segment> {
    let mut text = text;
    let mut data: Vec<Segment> = vec![];

    let regex_left = Regex::new(
        format!(
            "({})",
            delimiters
                .iter()
                .map(|delim| escape_replace(&delim.left))
                .collect::<Vec<String>>()
                .join("|")
        )
        .as_str(),
    )
    .unwrap();

    let regex_ams = Regex::new(r"^\\begin\{").unwrap();
    while let Some(m) = regex_left.find(text) {
        let start = m.start();
        let m_str = m.as_str();
        if start > 0 {
            // escape
            if text.as_bytes()[start - 1] == b'\\' {
                data.push(Segment::Text { data: text[..start - 1].to_string() + m_str });
                text = &text[start + m_str.len()..];
                continue;
            }
            data.push(Segment::Text { data: text[..start].to_string() });
            text = &text[start..];
        }

        let i = delimiters.iter().position(|d| d.left == m_str).unwrap();
        let delim = &delimiters[i];
        let end = match find_end_of_math(&delim.right, text, delim.left.len()) {
            Some(u) => u,
            // unclosed delimiter
            None => {
                data.push(Segment::Unclosed(m_str.to_string()));
                break;
            }
        };
        let raw_data = text[..end + delim.right.len()].to_string();
        let math = if regex_ams.is_match(&raw_data) {
            raw_data.clone()
        } else {
            text[m_str.len()..end].to_string()
        };
        data.push(Segment::Math { data: math, raw_data, display: delim.display });

        text = &text[end + delim.right.len()..];
    }

    if !text.is_empty() {
        data.push(Segment::Text { data: text.to_string() });
    }

    data
}

pub(crate) fn render_katex(content: &str, ctx: &KatexContext) -> Result<String> {
    if !ctx.enable {
        return Ok(content.to_string());
    }

    let datas = spilt_at_delimiters(content, &ctx.delimiters);
    if datas.len() == 1 {
        if let Segment::Text { data } = &datas[0] {
            return Ok(data.clone());
        }
    }

    let mut errors = vec![];
    let result = datas.iter().fold(String::new(), |result, s| match s {
        Segment::Text { data } => result + data,
        Segment::Math { data, raw_data, display } => {
            let rendered = render_katex_aux(data, *display);
            match rendered {
                Ok(str) => result + &str,
                Err(err) => {
                    errors.push(err.to_string());
                    result + &format!(" ***[KaTeX Warning] Fail to parse formula:*** {}", raw_data)
                }
            }
        }
        Segment::Unclosed(delim) => {
            errors.push(
                format!("find unclosed delimiter: \"{}\", KaTeX formula delimiter should be closed properly", delim)
            );
            result + " ***[KaTeX Warning] Unclosed delimiter ->*** "
        }
    });

    if ctx.restrict && !errors.is_empty() {
        return Err(Error::msg(errors.join("\n")));
    }

    //println!("render: \n{}", result);
    Ok(result)
}

pub(crate) fn render_katex_aux(content: &str, display: bool) -> katex::Result<String> {
    let k_opts = Opts::builder().display_mode(display).build().unwrap();
    katex::render_with_opts(content, k_opts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> KatexContext {
        KatexContext::new(
            true,
            false,
            vec![Delimiter::new("$$", "$$", true), Delimiter::new("$", "$", false)],
        )
    }

    fn unchanged(eg: &str) {
        assert_eq!(eg, render_katex(eg, &ctx()).unwrap());
    }

    fn changed(eg: &str) {
        let result = render_katex(eg, &ctx()).unwrap();
        assert!(result.len() > eg.len());
        assert_ne!(eg, &result[..eg.len()]);
    }

    #[test]
    fn no_math_unchanged() {
        unchanged("This is just a sentence.");
    }

    #[test]
    #[should_panic]
    fn restrict_working() {
        let ctx = KatexContext { restrict: true, ..ctx() };
        let _ = render_katex(r"$$katex", &ctx).unwrap();
    }

    #[test]
    fn unclosed_delimiter_warn() {
        assert_eq!(
            r"$F = ma ***[KaTeX Warning] Unclosed delimiter ->*** $",
            render_katex(r"\$F = ma$", &ctx()).unwrap()
        );
        assert_eq!(
            r" ***[KaTeX Warning] Unclosed delimiter ->*** $F = ma\$",
            render_katex(r"$F = ma\$", &ctx()).unwrap()
        );
    }

    #[test]
    fn delimiter_escape() {
        assert_eq!(r"$F = ma$", render_katex(r"\$F = ma\$", &ctx()).unwrap());
        assert_eq!(r"$$F = ma$", render_katex(r"\$$F = ma\$", &ctx()).unwrap());
    }

    #[test]
    fn working_inline() {
        let eg = r"Consider $π = \frac{1}{2}τ$ for a moment.";
        let result = render_katex(eg, &ctx()).unwrap();
        assert!(result.len() > eg.len());
        assert_ne!(eg, result);
        assert_eq!(eg[..9], result[..9]);
        assert_eq!(eg[eg.len() - 14..], result[result.len() - 14..]);
    }

    #[test]
    fn working_multiline() {
        changed(r"$$\sum_{i = 0}^n i = \frac{1}{2}n(n+1)$$");
        // N.B. trailing whitespace is deliberate and should not disable math mode.
        changed(
            r"    $$ 
        \sum_{i = 0}^n i = \frac{1}{2}n(n+1) 
    $$",
        );
    }

    #[test]
    fn multiple_formulae() {
        let eg = r"Consider $π = \frac{1}{2}τ$, then
            $$
                4 \int_{-1}^1 \sqrt{1 - x^2} \mathop{dx} = τ
            $$
            and also consider $A = πr^2$ for a moment.";
        let result = render_katex(eg, &ctx()).unwrap();
        assert!(result.len() > eg.len());
        assert!(result.contains(", then"));
        assert!(result.contains("and also consider "));
        assert_ne!(eg, result);
        assert_eq!(eg[..9], result[..9]);
        assert_eq!(eg[eg.len() - 14..], result[result.len() - 14..]);
    }
}
