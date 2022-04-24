// `pub const x: &str = "";`
// you can also include a file as a string
// `pub const x: &str = std::include_str!("x.y");`

// HTML template

pub const HTML: &'static str = std::include_str!("template.html");

// CSS
pub const CSS: &'static str = std::include_str!("style.css");

// Mathjax
//pub const MATHJAX_CONFIG: &'static str = std::include_str!("mathjax-config.js");
//pub const MATHJAX_SCRIPT: &'static str = std::include_str!("mathjax.min.js");

pub const MATHJAX_CONFIG: &'static str = "";
pub const MATHJAX_SCRIPT: &'static str = "";
