#![allow(unused)]
#[derive(Debug, PartialEq)]
pub enum InlineToken {
    Escape,
    // *
    Star,
    DoubleStar,
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
    DoubleStrike,
    // ==
    Equal,
    DoubleEqual,
    Plain(String)
}

use InlineToken::*;

fn tokenize(data: String) -> Vec<InlineToken> {
    data.chars().fold(vec![], |mut tokens, ch| {
        match ch {
            '/' => {
                tokens.push(Escape);
            },
            '*' => {
                if let Some(token) = tokens.last_mut().filter(|t| *t == &Star) {
                    *token = DoubleStar;
                } else {
                    tokens.push(Star);
                }
            },
            '`' => {
                tokens.push(Quote);
            },
            '[' => {
                tokens.push(SquareOpen);
            },
            ']' => {
                tokens.push(SquareClose);
            },
            '(' => {
                tokens.push(CircleOpen);
            },
            ')' => {
                tokens.push(CircleClose);
            },
            '^' => {
                if let Some(token) = tokens.last_mut().filter(|t| *t == &SquareOpen) {
                    *token = FootnoteOpen;
                } else {
                    tokens.push(Plain('^'.to_string()));
                }
            },
            '~' => {
                if let Some(token) = tokens.last_mut().filter(|t| *t == &Strike) {
                    *token = DoubleStar;
                } else {
                    tokens.push(Star);
                }
            },
            '=' => {
                if let Some(token) = tokens.last_mut().filter(|t| *t == &Equal) {
                    *token = DoubleEqual;
                } else {
                    tokens.push(Equal);
                }
            },
            ch =>  {
                if let Some(Plain(s)) = tokens.last_mut() {
                    s.push(ch);
                } else {
                    tokens.push(Plain(String::from(ch)));
                }
            },
        }
        tokens
    })
}
