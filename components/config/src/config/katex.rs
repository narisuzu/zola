use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delimiter {
    pub left: String,
    pub right: String,
    pub display: bool,
}

impl Delimiter {
    pub fn new(left: &str, right: &str, display: bool) -> Self {
        Delimiter { left: left.to_string(), right: right.to_string(), display }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Katex {
    // default `false`. it's global option, KaTeX formula won't be rendered once this is false.
    // to render katex, it's necessary to enable the 'katex' option in the frontmatter of each Page/Section as well.
    pub enable: bool,
    // css cdn url, must be set properly, or your formulas won't be rendered properly
    pub css_url: String,
    // css integrity, like above
    pub css_integrity: String,
    // default `false`, if `true`, wrong KaTeX formulas will cause panic. else it won't be rendered
    pub restrict: bool,
    // set the delimiters of formula. Use '\' for escaping
    // Example:
    // "ab\$cd\$ef" will be rendered to "ab$cd$ef". "cd\" won't be processed as a formula
    // "ab\$$cd\$$ef" -> "ab$$cd$$ef". delimiters composed by multiple chars can be escaped with a single '\' before it.
    pub delimiters: Vec<Delimiter>,
}

impl Default for Katex {
    fn default() -> Self {
        Katex {
            enable: false,
            restrict: false,
            delimiters: vec![Delimiter::new("$$", "$$", true), Delimiter::new("$", "$", false)],
            css_url: "https://cdn.jsdelivr.net/npm/katex@0.15.2/dist/katex.min.css".into(),
            css_integrity:
                "sha384-MlJdn/WNKDGXveldHDdyRP1R4CTHr3FeuDNfhsLPYrq2t0UBkUdK2jyTnXPEK1NQ".into(),
        }
    }
}
