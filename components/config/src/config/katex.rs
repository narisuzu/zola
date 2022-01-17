use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delimiter {
    pub left: String,
    pub right: String,
    pub display: bool,
}

impl Delimiter {
    fn new(left: &str, right: &str, display: bool) -> Self {
        Delimiter { left: left.to_string(), right: right.to_string(), display }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Katex {
    // default `false`. KaTeX formula will be rendered only with `true`
    pub enable: bool,
    // default `false`, if `true`, wrong KaTeX formula will cause panic. else it won't be rendered
    pub restrict: bool,
    // set the delimiters of formula. Use '\' for escaping
    // Example:
    // "ab\$cd\$ef" will be rendered to "ab$cd$ef". "cd" won't be treated as a formula
    // "ab\$$cd\$$ef" -> "ab$$cd$$ef"
    pub delimiters: Vec<Delimiter>,
}

impl Default for Katex {
    fn default() -> Self {
        Katex {
            enable: false,
            restrict: false,
            delimiters: vec![Delimiter::new("$$", "$$", true), Delimiter::new("$", "$", false)],
        }
    }
}
