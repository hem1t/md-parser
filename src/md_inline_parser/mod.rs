///
/// After Blocks have been parsed we come here
/// to find Inline Elements
///

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

#[derive(Debug)]
pub(crate) struct MdString {
    string: Vec<MdString>,
}
