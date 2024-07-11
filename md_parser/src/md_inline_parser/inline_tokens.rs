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

macro_rules! skip_if_escape {
    ($tokens:ident, $ch:ident) => {
        if let Some(token) = $tokens.last_mut().filter(|t| *t == &Escape) {
            *token = Plain($ch.to_string());
            continue;
        }
    };
}

use InlineToken::*;

fn tokenize(data: String) -> Vec<InlineToken> {
    let mut tokens = vec![];

    for ch in data.chars() {
        match ch {
            '\\' => {
                skip_if_escape!(tokens, ch);
                tokens.push(Escape);
            }
            '*' => {
                skip_if_escape!(tokens, ch);
                tokens.push(Star);
            }
            '`' => {
                skip_if_escape!(tokens, ch);
                tokens.push(Quote);
            }
            '[' => {
                skip_if_escape!(tokens, ch);
                tokens.push(SquareOpen);
            }
            ']' => {
                skip_if_escape!(tokens, ch);
                tokens.push(SquareClose);
            }
            '(' => {
                skip_if_escape!(tokens, ch);
                tokens.push(CircleOpen);
            }
            ')' => {
                skip_if_escape!(tokens, ch);
                tokens.push(CircleClose);
            }
            '^' => {
                skip_if_escape!(tokens, ch);
                if let Some(token) = tokens.last_mut().filter(|t| *t == &SquareOpen) {
                    *token = FootnoteOpen;
                } else {
                    tokens.push(Plain('^'.to_string()));
                }
            }
            '~' => {
                skip_if_escape!(tokens, ch);
                tokens.push(Strike);
            }
            '=' => {
                skip_if_escape!(tokens, ch);
                tokens.push(Equal);
            }
            ch => {
                skip_if_escape!(tokens, ch);
                if let Some(Plain(s)) = tokens.last_mut() {
                    s.push(ch);
                } else {
                    tokens.push(Plain(String::from(ch)));
                }
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
        vec![Plain("=bold".to_string()), Plain("~".to_string())]
    );
}

#[test]
fn test_inline_footnote() {
    assert_eq!(
        tokenize("[^1\\]".to_string()),
        vec![FootnoteOpen, Plain("1".to_string()), Plain("]".to_string())]
    );
    assert_eq!(
        tokenize("\\[^1]".to_string()),
        vec![Plain("[".to_string()), Plain("^1".to_string()), SquareClose]
    );
    assert_eq!(
        tokenize("[\\^1]".to_string()),
        vec![SquareOpen, Plain("^1".to_string()), SquareClose]
    );
}
