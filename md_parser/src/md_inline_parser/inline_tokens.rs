#[derive(Debug, PartialEq)]
pub enum InlineToken {
    Escape,
    // *
    Star,
    // \`
    Quote,
    // []
    SquareOpen,
    SquareClose,
    // ()
    CircleOpen,
    CircleClose,
    // [^
    FootnoteOpen,
    // ~
    Strike,
    // ==
    Equal,
    Plain(String),
}

use InlineToken::*;

use crate::md_inline_parser::VecLastMutIfMatch;

macro_rules! push_to_plain {
    ($tokens:ident, $ch:ident) => {
        if let Some(Plain(s)) = $tokens.last_mut() {
            s.push($ch);
        } else {
            $tokens.push(Plain(String::from($ch)));
        }
    };
}

fn tokenize(data: String) -> Vec<InlineToken> {
    let mut tokens = vec![];

    for ch in data.chars() {
        // if escape then skip
        if let Some(Escape) = tokens.last() {
            tokens.pop();
            push_to_plain!(tokens, ch);
            continue;
        }
        match ch {
            '\\' => {
                tokens.push(Escape);
            }
            '*' => {
                tokens.push(Star);
            }
            '`' => {
                tokens.push(Quote);
            }
            '[' => {
                tokens.push(SquareOpen);
            }
            ']' => {
                tokens.push(SquareClose);
            }
            '(' => {
                tokens.push(CircleOpen);
            }
            ')' => {
                tokens.push(CircleClose);
            }
            '^' => {
                if let Some(token) = tokens.last_mut_if(|t| *t == SquareOpen) {
                    *token = FootnoteOpen;
                } else {
                    push_to_plain!(tokens, ch);
                }
            }
            '~' => {
                tokens.push(Strike);
            }
            '=' => {
                tokens.push(Equal);
            }
            ch => {
                push_to_plain!(tokens, ch);
            }
        }
    }

    // NOTE: returning
    tokens
}

#[test]
fn test_inline_tokens() {
    assert_eq!(
        tokenize("`code **bold**`".to_string()),
        vec![
            Quote,
            Plain("code ".to_string()),
            Star,
            Star,
            Plain("bold".to_string()),
            Star,
            Star,
            Quote
        ]
    );
}

#[test]
fn test_inline_star() {
    assert_eq!(
        tokenize("**bold**".to_string()),
        vec![Star, Star, Plain("bold".to_string()), Star, Star]
    );
    assert_eq!(
        tokenize("*bold*".to_string()),
        vec![Star, Plain("bold".to_string()), Star]
    );
}

#[test]
fn test_inline_equal() {
    assert_eq!(
        tokenize("==bold==".to_string()),
        vec![Equal, Equal, Plain("bold".to_string()), Equal, Equal]
    );
    assert_eq!(
        tokenize("=bold=".to_string()),
        vec![Equal, Plain("bold".to_string()), Equal]
    );
}

#[test]
fn test_inline_escape() {
    assert_eq!(
        tokenize("\\**bold**".to_string()),
        vec![
            Plain("*".to_string()),
            Star,
            Plain("bold".to_string()),
            Star,
            Star
        ]
    );
    assert_eq!(
        tokenize("\\=bold=".to_string()),
        vec![Plain("=bold".to_string()), Equal]
    );
    assert_eq!(
        tokenize("\\=bold\\~".to_string()),
        vec![Plain("=bold~".to_string())]
    );
}

#[test]
fn test_inline_footnote() {
    assert_eq!(
        tokenize("[^1\\]".to_string()),
        vec![FootnoteOpen, Plain("1]".to_string())]
    );
    assert_eq!(
        tokenize("\\[^1]".to_string()),
        vec![Plain("[^1".to_string()), SquareClose]
    );
    assert_eq!(
        tokenize("[\\^1]".to_string()),
        vec![SquareOpen, Plain("^1".to_string()), SquareClose]
    );

}
