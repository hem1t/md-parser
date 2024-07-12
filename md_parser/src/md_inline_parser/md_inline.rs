use std::slice::Iter;

use crate::md_inline_parser::inline_tokens;

use super::{inline_tokens::InlineToken, md_string::MdString, VecLastMutIfMatch};

#[derive(Debug, PartialEq)]
pub(crate) enum MdInline {
    Bold(Box<MdString>),
    Italic(Box<MdString>),
    BoldItalic(Box<MdString>),
    Code(Box<MdString>),
    Strike(Box<MdString>),
    Highlight(Box<MdString>),
    Sub(Box<MdString>),
    Super(Box<MdString>),
    Link(String, String),
    InlineString(String),
}

use MdInline::*;

impl MdInline {
    pub fn escape(&self) -> InlineString(String) {
        match self {

        }
    }
}

pub fn from_tokens_to_mdinline(
    tokens: &mut Iter<'_, InlineToken>,
    md_string: &mut MdString,
    until: Option<InlineToken>,
) {
    let token = tokens.next();
    if token.is_none() || until.as_ref() == token {
        return;
    }

    let token = token.unwrap();

    macro_rules! enclosed_matches {
        ($till:expr, $make:expr) => {{
            let mut i_md_string = MdString::new();
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some($till));
            md_string.push($make(Box::new(i_md_string)))
        }};
    }

    macro_rules! escaped_enclosed_matches {
        ($till:expr, $make:expr) => {{
            let mut i_md_string = MdString::new();
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some($till));
            i_md_string.escape();
            md_string.push($make(Box::new(i_md_string)))
        }};
    }

    match token {
        InlineToken::Escape => (),
        InlineToken::Star => enclosed_matches!(InlineToken::Star, MdInline::Italic),
        InlineToken::DoubleStar => enclosed_matches!(InlineToken::DoubleStar, MdInline::Bold),
        InlineToken::TripleStar => enclosed_matches!(InlineToken::TripleStar, MdInline::BoldItalic),
        InlineToken::DoubleStrike => enclosed_matches!(InlineToken::DoubleStrike, MdInline::Strike),
        InlineToken::DoubleEqual => {
            enclosed_matches!(InlineToken::DoubleEqual, MdInline::Highlight)
        }
        InlineToken::Strike => enclosed_matches!(InlineToken::Strike, MdInline::Super),
        InlineToken::Carat => enclosed_matches!(InlineToken::Carat, MdInline::Sub),
        InlineToken::Quote => escaped_enclosed_matches!(InlineToken::Quote, MdInline::Code),
        InlineToken::DoubleQuote => escaped_enclosed_matches!(InlineToken::DoubleQuote, MdInline::Code),
        InlineToken::SquareOpen => {
            
        }
        InlineToken::SquareClose => (),
        InlineToken::CircleOpen => {}
        InlineToken::CircleClose => (),
        InlineToken::FootnoteOpen => todo!(),
        InlineToken::Equal => md_string.push(InlineString(String::from('='))),
        InlineToken::Plain(f) => {
            if let Some(InlineString(s)) = md_string.last_mut() {
                s.push_str(f.as_str())
            } else {
                md_string.push(InlineString(f.to_owned()));
            }
        }
    }
    from_tokens_to_mdinline(tokens, md_string, until);
}

#[test]
fn test_mdline_plain() {
    let mut md_string = vec![];
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("Hello World!".to_string()).iter()),
        &mut md_string,
        None,
    );
    assert_eq!(
        md_string,
        vec![MdInline::InlineString("Hello World!".to_string())]
    );
}

#[test]
fn test_mdline_string_modifiers() {
    let mut md_string = vec![];
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("Hello \\**dkjf**world\\** !".to_string()).iter()),
        &mut md_string,
        None,
    );

    let result = vec![
        InlineString("Hello *".to_string()),
        Italic(Box::new(vec![
            InlineString("dkjf".to_string()),
            Bold(Box::new(vec![
                InlineString("world*".to_string()),
                Italic(Box::new(vec![InlineString(" !".to_string())])),
            ])),
        ])),
    ];

    assert_eq!(md_string, result);
}

#[test]
fn test_mdline_italic_bold() {
    let mut md_string = vec![];
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("***Hello Italic & Bold***".to_string()).iter()),
        &mut md_string,
        None,
    );

    let result = vec![BoldItalic(Box::new(vec![InlineString("Hello Italic & Bold".to_string())]))];
    assert_eq!(md_string, result);
}
