#[derive(Debug, PartialEq)]
pub enum InlineToken {
    Escape,
    // *
    Star,
    DoubleStar,
    TripleStar,
    // \`
    Quote,
    DoubleQuote,
    // []
    SquareOpen,
    SquareClose,
    // ()
    CircleOpen,
    CircleClose,
    // [^
    Carat,
    FootnoteOpen,
    // ~
    Strike,
    DoubleStrike,
    // ==
    Equal,
    DoubleEqual,
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

pub(crate) fn tokenize(data: String) -> Vec<InlineToken> {
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
                match tokens.last_mut() {
                    Some(token) if *token == Star => *token = DoubleStar,
                    Some(token) if *token == DoubleStar => *token = TripleStar,
                    _ => tokens.push(Star)
                }
            }
            '`' => {
                match tokens.last_mut() {
                    Some(token) if *token == Quote => *token = DoubleQuote,
                    _ => tokens.push(Quote)
                }
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
                match tokens.last_mut() {
                    Some(token) if *token == SquareOpen => *token = FootnoteOpen,
                    _ => tokens.push(Carat)
                }
            }
            '~' => {
                match tokens.last_mut() {
                    Some(token) if *token == Strike => *token = DoubleStrike,
                    _ => tokens.push(Strike)
                }
            }
            '=' => {
                match tokens.last_mut() {
                    Some(token) if *token == Equal => *token = DoubleEqual,
                    _ => tokens.push(Equal)
                }
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
            DoubleStar,
            Plain("bold".to_string()),
            DoubleStar,
            Quote,
        ]
    );
}

#[test]
fn test_inline_star() {
    assert_eq!(
        tokenize("**bold**".to_string()),
        vec![DoubleStar, Plain("bold".to_string()), DoubleStar]
    );
    assert_eq!(
        tokenize("*bold*".to_string()),
        vec![Star, Plain("bold".to_string()), Star]
    );
    assert_eq!(
        tokenize("***bold***".to_string()),
        vec![TripleStar, Plain("bold".to_string()), TripleStar]
    );
}

#[test]
fn test_inline_equal() {
    assert_eq!(
        tokenize("==bold==".to_string()),
        vec![DoubleEqual, Plain("bold".to_string()), DoubleEqual]
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
            DoubleStar
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
        vec![Plain("[".to_string()), Carat, Plain("1".to_string()), SquareClose]
    );
    assert_eq!(
        tokenize("[\\^1]".to_string()),
        vec![SquareOpen, Plain("^1".to_string()), SquareClose]
    );

}

// to_string impl
impl InlineToken {
    pub fn to_string(&self) -> String {
        match self {
            Escape => "\\".to_string(),
            Star => "*".to_string(),
            DoubleStar => "**".to_string(),
            TripleStar => "***".to_string(),
            Quote => "`".to_string(),
            DoubleQuote => "``".to_string(),
            SquareOpen => "[".to_string(),
            SquareClose => "]".to_string(),
            CircleOpen => "(".to_string(),
            CircleClose => ")".to_string(),
            Carat => "^".to_string(),
            FootnoteOpen => "[^".to_string(),
            Strike => "~".to_string(),
            DoubleStrike => "~~".to_string(),
            Equal => "=".to_string(),
            DoubleEqual => "==".to_string(),
            Plain(s) => s.to_string(),
        }
    }
}
