use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Katex {
    /// css cdn url, must be set properly, or your formulas won't be rendered properly
    pub css_url: String,
    /// css integrity, like above
    pub css_integrity: String,
    /// default `false`, if `true`, wrong KaTeX formulas will cause panic. else it won't be rendered
    pub restrict: bool,
    /// Wether rendering KaTeX code block
    pub codeblock: bool,
    /// Global macros
    pub macros: HashMap<String, String>,
}

impl Default for Katex {
    fn default() -> Self {
        Katex {
            codeblock: true,
            restrict: false,
            css_url: "https://cdn.jsdelivr.net/npm/katex@0.15.2/dist/katex.min.css".into(),
            css_integrity:
                "sha384-MlJdn/WNKDGXveldHDdyRP1R4CTHr3FeuDNfhsLPYrq2t0UBkUdK2jyTnXPEK1NQ".into(),
            macros: HashMap::new(),
        }
    }
}
