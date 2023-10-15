use std::collections::VecDeque;

use anyhow::{anyhow, bail, Result};

use crate::compiler::lexer::token::{Keyword, Token};

pub mod ast;

pub fn parse(tokens: Vec<Token>) -> Result<ast::Root> {
    let parser = Parser {
        tokens: VecDeque::from(tokens),
    };

    parser.parse_root()
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    fn peek_token(&self) -> Option<&Token> {
        self.tokens.iter().next()
    }

    fn eat_token(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    fn expect_token(&mut self, expected: Token) -> Result<()> {
        let token = self.eat_token().ok_or_else(|| {
            anyhow!("Expected token `{expected:?}, reached end of token stream instead`")
        })?;

        if token != expected {
            bail!("Expected token `{expected:?}`, got `{token:?}` instead")
        }

        Ok(())
    }

    fn parse_compound(&mut self) -> Result<ast::CompoundExpr> {
        self.expect_token(Token::LeftBrace)?;

        let mut expressions = Vec::new();

        loop {
            if let Some(Token::LeftBrace) = self.peek_token() {
                let sub_compound = self.parse_compound()?;
                expressions.push(ast::Expr::Compound(Box::new(sub_compound)));
            }

            let expr = match self.eat_token() {
                Some(Token::Keyword(Keyword::Exit)) => {
                    if let Some(Token::Number(exit_code)) = self.eat_token() {
                        ast::Expr::Exit(exit_code)
                    } else {
                        bail!("Expected numeric exit code after exit keyword");
                    }
                }
                Some(Token::RightBrace) => break,
                Some(Token::NewLine) => continue,
                Some(Token::Identifier(fn_name)) => {
                    self.expect_token(Token::LeftParentheses)?;
                    self.expect_token(Token::RightParentheses)?;
                    ast::Expr::FnCall(fn_name)
                }
                Some(other) => bail!("Got invalid token `{other:?}` in compound expression"),
                None => bail!("Expected token, reached end of token stream instead"),
            };

            expressions.push(expr);
        }

        Ok(ast::CompoundExpr { expressions })
    }

    fn parse_function_def(&mut self) -> Result<ast::FunctionDefinition> {
        self.expect_token(Token::Keyword(Keyword::Fun))?;

        let name = match self.eat_token() {
            Some(Token::Identifier(name)) => name,
            Some(other) => bail!("Expected function identifier, got {other:?} instead"),
            None => bail!("Expected function identifier, reached end of token stream instead"),
        };

        self.expect_token(Token::LeftParentheses)?;
        self.expect_token(Token::RightParentheses)?;

        let compound = self.parse_compound()?;

        Ok(ast::FunctionDefinition { name, compound })
    }

    fn skip_newlines(&mut self) {
        while let Some(&Token::NewLine) = self.peek_token() {
            self.eat_token();
        }
    }

    fn parse_root(mut self) -> Result<ast::Root> {
        let mut functions = Vec::new();
        loop {
            self.skip_newlines();
            if let Some(tok) = self.peek_token() {
                functions.push(self.parse_function_def()?);
            } else {
                break;
            }
        }

        let function_names: Vec<String> = functions.iter().map(|f| f.name.clone()).collect();

        Ok(ast::Root {
            function_names,
            functions,
        })
    }
}
