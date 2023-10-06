use std::iter;

use anyhow::{bail, Result};

use crate::compiler::token;
use crate::compiler::token::Token;

pub struct Lexer<'a> {
    remaining_src_code: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(src_code: &'a str) -> Self {
        Self {
            remaining_src_code: src_code,
        }
    }

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
        while self.peek_char().map_or(false, |c| c.is_whitespace() && c != '\n' && c != '\r') {
            self.eat_char();
        }
    }

    fn try_read_identifier(&mut self) -> Option<String> {
        self.peek_char().filter(|c| c.is_alphabetic()).map(|first_char| {
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

        // TODO try_read_number

        // Read all other token types
        self.eat_char().map(|c| {
            let token = match c {
                '[' => LeftSquareBracket,
                ']' => RightSquareBracket,
                '(' => LeftParentheses,
                ')' => RightParentheses,
                '{' => LeftBrace,
                '}' => RightBrace,
                ';' | '\n' => NewLine,
                '\r' => {
                    self.expect_char('\n')?;
                    NewLine
                }
                invalid_char => bail!("Invalid character `{invalid_char}`"),
            };

            Ok(token)
        }).transpose()
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        iter::from_fn(|| self.next_token().transpose()).collect()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::compiler::lexer::Lexer;
    use crate::compiler::token::{Keyword, Token};

    #[test]
    pub fn empty_src() -> Result<()> {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.next_token()?, None);
        assert_eq!(lexer.next_token()?, None);

        Ok(())
    }

    #[test]
    pub fn brackets() -> Result<()> {
        let mut lexer = Lexer::new("([{}])");

        assert_eq!(lexer.next_token()?, Some(Token::LeftParentheses));
        assert_eq!(lexer.next_token()?, Some(Token::LeftSquareBracket));
        assert_eq!(lexer.next_token()?, Some(Token::LeftBrace));
        assert_eq!(lexer.next_token()?, Some(Token::RightBrace));
        assert_eq!(lexer.next_token()?, Some(Token::RightSquareBracket));
        assert_eq!(lexer.next_token()?, Some(Token::RightParentheses));

        Ok(())
    }

    #[test]
    pub fn identifiers() -> Result<()> {
        let mut lexer = Lexer::new("some identifier123 a_b");

        assert_eq!(lexer.next_token()?, Some(Token::Identifier("some".to_owned())));
        assert_eq!(lexer.next_token()?, Some(Token::Identifier("identifier123".to_owned())));
        assert_eq!(lexer.next_token()?, Some(Token::Identifier("a_b".to_owned())));

        Ok(())
    }

    #[test]
    pub fn empty_main() -> Result<()> {
        let mut lexer = Lexer::new("fun main() {\n}");

        assert_eq!(lexer.next_token()?, Some(Token::Keyword(Keyword::Fun)));
        assert_eq!(lexer.next_token()?, Some(Token::Identifier("main".to_owned())));
        assert_eq!(lexer.next_token()?, Some(Token::LeftParentheses));
        assert_eq!(lexer.next_token()?, Some(Token::RightParentheses));
        assert_eq!(lexer.next_token()?, Some(Token::LeftBrace));
        assert_eq!(lexer.next_token()?, Some(Token::NewLine));
        assert_eq!(lexer.next_token()?, Some(Token::RightBrace));
        assert_eq!(lexer.next_token()?, None);

        Ok(())
    }
}