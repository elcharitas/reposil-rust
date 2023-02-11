use std::iter::Peekable;
use std::str::Chars;

use crate::enums::Token;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let chars = &mut self.chars;

        loop {
            match chars.peek() {
                None => return None,
                Some(&';') => {
                    chars.next();
                    return Some(Token::Semicolon);
                }
                Some(&'\n') => {
                    chars.next();
                    return Some(Token::Semicolon);
                }
                Some(&c) if c.is_whitespace() => {
                    chars.next();
                    continue;
                }
                Some(&c) if c.is_digit(10) => {
                    let mut number = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_digit(10) || c == '.' {
                            number.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Number(number.parse().unwrap()));
                }
                Some(&'p') => {
                    chars.next();
                    if let Some(&'r') = chars.peek() {
                        chars.next();
                        if let Some(&'i') = chars.peek() {
                            chars.next();
                            if let Some(&'n') = chars.peek() {
                                chars.next();
                                if let Some(&'t') = chars.peek() {
                                    chars.next();
                                    return Some(Token::Print);
                                }
                            }
                        }
                    }
                    return None;
                }
                Some(&c) if c.is_alphabetic() => {
                    let mut identifier = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_alphanumeric() {
                            identifier.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Identifier(identifier));
                }
                Some(&'+') => {
                    chars.next();
                    return Some(Token::Plus);
                }
                Some(&'-') => {
                    chars.next();
                    return Some(Token::Minus);
                }
                Some(&'*') => {
                    chars.next();
                    return Some(Token::Star);
                }
                Some(&'/') => {
                    chars.next();
                    return Some(Token::Slash);
                }
                Some(&'(') => {
                    chars.next();
                    return Some(Token::LParen);
                }
                Some(&')') => {
                    chars.next();
                    return Some(Token::RParen);
                }
                Some(&'=') => {
                    chars.next();
                    return Some(Token::Equal);
                }
                Some(&'.') => {
                    chars.next();
                    return Some(Token::Dot);
                }
                _ => return None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new("1 + 2 - 3 * 4 / 5");
        assert_eq!(lexer.next_token(), Some(Token::Number(1.0)));
        assert_eq!(lexer.next_token(), Some(Token::Plus));
        assert_eq!(lexer.next_token(), Some(Token::Number(2.0)));
        assert_eq!(lexer.next_token(), Some(Token::Minus));
        assert_eq!(lexer.next_token(), Some(Token::Number(3.0)));
        assert_eq!(lexer.next_token(), Some(Token::Star));
        assert_eq!(lexer.next_token(), Some(Token::Number(4.0)));
        assert_eq!(lexer.next_token(), Some(Token::Slash));
        assert_eq!(lexer.next_token(), Some(Token::Number(5.0)));
        assert_eq!(lexer.next_token(), None);
    }
}
