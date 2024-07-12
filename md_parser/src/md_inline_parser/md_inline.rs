use std::slice::Iter;

use crate::md_inline_parser::inline_tokens;

use super::{inline_tokens::InlineToken, md_string::MdString, VecLastMutIfMatch};

#[derive(Debug, PartialEq)]
pub(crate) enum MdInline {
    Bold(MdString),
    Italic(MdString),
    BoldItalic(MdString),
    Code(MdString),
    Strike(MdString),
    Highlight(MdString),
    Sub(MdString),
    Super(MdString),
    LinkText(MdString),
    LinkUrl(MdString),
    Footnote(MdString),
    InlineString(String),
}

use MdInline::*;

pub fn from_tokens_to_mdinline(
    tokens: &mut Iter<'_, InlineToken>,
    md_string: &mut MdString,
    until: Option<InlineToken>,
    escape: bool,
) {
    let token = tokens.next();
    // token == matters, here don't move it
    if token.is_none() || until.as_ref() == token {
        return;
    }

    let token = token.unwrap();

    macro_rules! enclosed_matches {
        ($till:expr, $make:expr) => {{
            let mut i_md_string = MdString::new();
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some($till), false);
            md_string.push($make(i_md_string));
        }};
    }

    macro_rules! escaped_enclosed_matches {
        ($till:expr, $make:expr) => {{
            let mut i_md_string = MdString::new();
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some($till), true);
            // TODO: merge i_md_string
            md_string.push($make(i_md_string));
        }};
    }

    // Inits go above this.
    // escaping
    if escape {
        md_string.push(MdInline::InlineString(token.to_string()));
        from_tokens_to_mdinline(tokens, md_string, until, true);
        return;
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
        InlineToken::DoubleQuote => {
            escaped_enclosed_matches!(InlineToken::DoubleQuote, MdInline::Code)
        }
        InlineToken::SquareOpen => enclosed_matches!(InlineToken::SquareClose, MdInline::LinkText),
        InlineToken::SquareClose => (),
        InlineToken::CircleOpen => {
            escaped_enclosed_matches!(InlineToken::SquareClose, MdInline::LinkUrl)
        }
        InlineToken::CircleClose => (),
        InlineToken::FootnoteOpen => {
            escaped_enclosed_matches!(InlineToken::SquareClose, MdInline::Footnote)
        }
        InlineToken::Equal => md_string.push(InlineString(String::from('='))),
        InlineToken::Plain(f) => {
            if let Some(InlineString(s)) = md_string.last_mut() {
                s.push_str(f.as_str())
            } else {
                md_string.push(InlineString(f.to_owned()));
            }
        }
    }
    from_tokens_to_mdinline(tokens, md_string, until, false);
}

#[test]
fn test_mdline_plain() {
    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("Hello World!".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );
    assert_eq!(
        md_string,
        MdString::from_vec(vec![MdInline::InlineString("Hello World!".to_string())])
    );
}

#[test]
fn test_mdline_string_modifiers() {
    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("Hello \\**dkjf**world\\** !".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );

    let result = MdString::from_vec(vec![
        MdInline::InlineString("Hello *".to_string()),
        MdInline::Italic(MdString::from_vec(vec![
            InlineString("dkjf".to_string()),
            Bold(MdString::from_vec(vec![
                InlineString("world*".to_string()),
                Italic(MdString::from_vec(vec![InlineString(" !".to_string())])),
            ])),
        ])),
    ]);

    assert_eq!(md_string, result);
}

#[test]
fn test_mdline_italic_bold() {
    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("***Hello Italic & Bold***".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );

    let result = MdString::from_vec(vec![BoldItalic(MdString::from_vec(vec![InlineString(
        "Hello Italic & Bold".to_string(),
    )]))]);
    assert_eq!(md_string, result);
}

#[test]
fn test_mdline_link() {
    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("[**bold text**]".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );

    let result = MdString::from_vec(vec![LinkText(MdString::from_vec(vec![Bold(
        MdString::from_vec(vec![InlineString("bold text".to_string())]),
    )]))]);
    assert_eq!(md_string, result);

    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("(**bold text**)".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );

    let result = MdString::from_vec(vec![MdInline::LinkUrl(MdString::from_vec(vec![
        InlineString("**bold text**".to_string()),
    ]))]);
    assert_eq!(md_string, result);
}

#[test]
fn test_mdline_footnote() {
    let mut md_string = MdString::new();
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("[^123]".to_string()).iter()),
        &mut md_string,
        None,
        false,
    );

    let result = MdString::from_vec(vec![MdInline::Footnote(MdString::from_vec(vec![
        MdInline::InlineString("123".to_string()),
    ]))]);
    assert_eq!(md_string, result);
}
