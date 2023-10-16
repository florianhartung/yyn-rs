use std::iter;

use anyhow::{bail, Result};
use itertools::Itertools;

use token::Token;

pub mod token;

pub fn tokenize(src: &str) -> Result<Vec<Token>> {
    let mut lexer = Lexer {
        remaining_src_code: src,
    };
    iter::from_fn(|| lexer.next_token().transpose()).collect()
}

pub struct Lexer<'a> {
    remaining_src_code: &'a str,
}

impl<'a> Lexer<'a> {
    fn peek_char(&self) -> Option<char> {
        self.remaining_src_code.chars().next()
    }

    fn eat_char(&mut self) -> Option<char> {
        let mut chars = self.remaining_src_code.chars();

        let first_char = chars.next();
        self.remaining_src_code = chars.as_str();

        first_char
    }

    fn expect_char(&mut self, expected: char) -> Result<()> {
        let c = self.eat_char();
        if c != Some(expected) {
            if let Some(c) = c {
                bail!("Expected char `{expected}`, got `{c}`")
            } else {
                bail!("Expected char `{expected}`, reached end of file instead")
            }
        } else {
            Ok(())
        }
    }

    fn skip_whitespaces(&mut self) {
        const IGNORE_CHARS: [char; 2] = ['\n', '\r'];

        while let Some(c) = self.peek_char() {
            if c.is_whitespace() && !IGNORE_CHARS.contains(&c) {
                self.eat_char();
            } else {
                break;
            }
        }
    }

    fn try_read_number(&mut self) -> Option<u32> {
        self.peek_char()
            .filter(|c| c.is_numeric())
            .map(|first_char| {
                let _ = self.eat_char();
                let mut number_value = String::from(first_char);

                while let Some(c) = self.peek_char() {
                    if c.is_numeric() {
                        let _ = self.eat_char();
                        number_value.push(c);
                    } else {
                        break;
                    }
                }

                number_value
                    .parse::<u32>()
                    .expect("number to be valid because it consists of only numerics")
            })
    }

    fn try_read_identifier(&mut self) -> Option<String> {
        self.peek_char()
            .filter(|c| c.is_alphabetic())
            .map(|first_char| {
                let _ = self.eat_char();
                let mut identifier = String::from(first_char);

                while let Some(c) = self.peek_char() {
                    match c {
                        c if c.is_alphanumeric() || c == '_' => {
                            let _ = self.eat_char();
                            identifier.push(c);
                        }
                        _ => break,
                    }
                }

                identifier
            })
    }

    pub fn next_token(&mut self) -> Result<Option<Token>> {
        use Token::*;

        self.skip_whitespaces();

        if let Some(identifier_name) = self.try_read_identifier() {
            let keyword = token::Keyword::try_from_str(identifier_name.as_str());

            let token = if let Some(keyword) = keyword {
                Keyword(keyword)
            } else {
                Identifier(identifier_name)
            };

            return Ok(Some(token));
        }

        if let Some(number) = self.try_read_number() {
            return Ok(Some(Number(number)));
        }

        // Read all other token types
        self.eat_char()
            .map(|c| {
                Ok(match c {
                    '[' => LeftSquareBracket,
                    ']' => RightSquareBracket,
                    '(' => LeftParentheses,
                    ')' => RightParentheses,
                    '{' => LeftBrace,
                    '}' => RightBrace,
                    ';' | '\n' => NewLine,
                    '-' => {
                        self.expect_char('>')?;
                        RightArrow
                    }
                    '\r' => {
                        self.expect_char('\n')?;
                        NewLine
                    }
                    invalid_char => bail!("Invalid character `{invalid_char}`"),
                })
            })
            .transpose()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::compiler::lexer::token::Token;
    use crate::compiler::lexer::tokenize;

    #[test]
    pub fn empty_src() -> Result<()> {
        let tokens = tokenize("")?;

        assert_eq!(tokens.as_slice(), &[]);

        Ok(())
    }

    #[test]
    pub fn brackets() -> Result<()> {
        let tokens = tokenize("([{}])")?;

        assert_eq!(
            tokens.as_slice(),
            &[
                Token::LeftParentheses,
                Token::LeftSquareBracket,
                Token::LeftBrace,
                Token::RightBrace,
                Token::RightSquareBracket,
                Token::RightParentheses,
            ]
        );

        Ok(())
    }

    #[test]
    pub fn identifiers() -> Result<()> {
        let tokens = tokenize("some identifier123 a_b")?;

        assert_eq!(
            tokens.as_slice(),
            &[
                Token::Identifier("some".to_owned()),
                Token::Identifier("identifier123".to_owned()),
                Token::Identifier("a_b".to_owned()),
            ]
        );

        Ok(())
    }
}
