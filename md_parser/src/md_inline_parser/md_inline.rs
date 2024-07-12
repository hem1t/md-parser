
use std::slice::Iter;

use crate::md_inline_parser::inline_tokens;

use super::{inline_tokens::InlineToken, VecLastMutIfMatch};

#[derive(Debug, PartialEq)]
pub(crate) enum MdInline {
    Bold(Box<Vec<MdInline>>),
    Italic(Box<Vec<MdInline>>),
    Code(Box<Vec<MdInline>>),
    Link(String, String),
    Strike(Box<Vec<MdInline>>),
    Highlight(Box<Vec<MdInline>>),
    Sub(Box<Vec<MdInline>>),
    Super(Box<Vec<MdInline>>),
    InlineString(String),
    NoMatch,
}

use MdInline::*;

//TODO: until needs to be a stack
pub fn from_tokens_to_mdinline(
    tokens: &mut Iter<'_, InlineToken>,
    md_string: &mut Vec<MdInline>,
    until: Option<InlineToken>,
) {
    let token = tokens.next();
    if token.is_none() || until.as_ref() == token {
        return;
    }

    let token = token.unwrap();

    match token {
        InlineToken::Star => {
            let mut i_md_string = vec![];
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some(InlineToken::Star));
            md_string.push(Italic(Box::new(i_md_string)))
        }
        InlineToken::DoubleStar => {
            let mut i_md_string = vec![];
            from_tokens_to_mdinline(tokens, &mut i_md_string, Some(InlineToken::DoubleStar));
            md_string.push(Bold(Box::new(i_md_string)))
        },
        InlineToken::DoubleStrike => todo!(),
        InlineToken::DoubleEqual => todo!(),
        InlineToken::Escape => (),
        InlineToken::Quote => todo!(),
        InlineToken::SquareOpen => todo!(),
        InlineToken::SquareClose => todo!(),
        InlineToken::CircleOpen => todo!(),
        InlineToken::CircleClose => todo!(),
        InlineToken::FootnoteOpen => todo!(),
        InlineToken::Strike => todo!(),
        InlineToken::Equal => md_string.push(InlineString(String::from('='))),
        InlineToken::Plain(f) => {
            if let Some(InlineString(s)) = md_string.last_mut() {
                s.push_str(f.as_str())
            } else {
                md_string.push(InlineString(f.to_owned()));
            }
        },
    }
    from_tokens_to_mdinline(tokens, md_string, until);
}

#[test]
fn test_mdline_plain() {
    let mut md_string = vec![];
    from_tokens_to_mdinline(
        dbg!(&mut inline_tokens::tokenize("Hello \\**dkjf**world\\** !".to_string()).iter()),
        &mut md_string,
        None,
    );
    assert_eq!(md_string, vec![MdInline::NoMatch]);
}
