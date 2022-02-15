use std::collections::HashMap;

use errors::{Error, Result};
use katex::{render_with_opts, Error as KatexError, Opts};

const TEX_WARN: &str = r#"<span class="katex-warning">
Invalid KaTeX eqation: <code>%CODE%</code>
</span>"#;

#[derive(Debug, Default)]
pub struct KatexContext {
    pub restrict: bool,
    pub display: bool,
    pub macros: HashMap<String, String>,
}

impl KatexContext {
    pub fn new(restrict: bool, display: bool, macros: HashMap<String, String>) -> Self {
        KatexContext { restrict, display, macros }
    }
}

pub fn render_katex(str: &str, ctx: &KatexContext) -> Result<String> {
    let opts =
        Opts::builder().display_mode(ctx.display).macros(ctx.macros.clone()).build().unwrap();
    let result = render_with_opts(str, opts);
    match result {
        Ok(str) => Ok(str),
        Err(KatexError::JsExecError(_)) if !ctx.restrict => Ok(TEX_WARN.replace("%CODE%", str)),
        Err(err) => Err(Error::msg(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macros_works() {
        let mut macros = HashMap::new();
        macros.insert(r"\RR".to_string(), r"\mathbb{R}".to_string());
        let ctx = KatexContext::new(true, true, macros);
        let result = render_katex(r"\forall x \in \RR", &ctx).unwrap();
        assert!(result.starts_with(r#"<span class="katex-display">"#))
    }

    #[test]
    fn render_succ() {
        let ctx = KatexContext::new(true, true, HashMap::new());
        let result = render_katex(r"a = \frac{p}{q}", &ctx).unwrap();
        assert!(result.starts_with(r#"<span class="katex-display">"#))
    }

    #[test]
    fn syntax_error_unstrict() {
        let ctx = KatexContext::new(false, true, HashMap::new());
        let result = render_katex(r"a \= \frac{p}{q}", &ctx).unwrap();
        assert!(result.starts_with(r#"<span class="katex-warning">"#))
    }

    #[test]
    #[should_panic]
    fn syntax_error_strict() {
        let ctx = KatexContext::new(true, true, HashMap::new());
        let _ = render_katex(r"a \= \frac{p}{q}", &ctx).unwrap();
    }
}
