#[derive(Debug)]
pub(crate) enum MdInline {
    Bold(String),
    Italic(String),
    Code(String),
    Link(String, String),
    Strike(String),
    Highlight(String),
    Sub(String),
    Super(String),
    Plain(String)
}
