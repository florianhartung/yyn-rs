use std::collections::VecDeque;

use anyhow::{anyhow, bail, Result};

use crate::compiler::lexer::token::{Keyword, Token};
use crate::compiler::parser::ast::Type;
use crate::compiler::symbol_table::Function;
use crate::compiler::symbol_table::Sym;

pub mod ast;

pub fn parse(tokens: Vec<Token>, sym: Sym) -> Result<ast::Root> {
    let parser = Parser {
        tokens: VecDeque::from(tokens),
        sym,
    };

    parser.parse_root()
}

pub struct Parser {
    tokens: VecDeque<Token>,
    sym: Sym,
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

    fn has_tokens(&self) -> bool {
        self.peek_token().is_some()
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
                Some(Token::Keyword(Keyword::Return)) => {
                    let Some(Token::Number(num)) = self.eat_token() else {
                        bail!("Expected number after return statement");
                    };
                    ast::Expr::Return(num)
                }
                Some(other) => bail!("Got invalid token `{other:?}` in compound expression"),
                None => bail!("Expected token, reached end of token stream instead"),
            };

            expressions.push(expr);
        }

        Ok(ast::CompoundExpr { expressions })
    }

    fn parse_type(&mut self) -> Result<ast::Type> {
        let Some(tok) = self.eat_token() else {
            bail!("Expected token for type definition");
        };

        Ok(match tok {
            Token::Keyword(Keyword::Int) => ast::Type::Int,
            Token::LeftParentheses => {
                self.expect_token(Token::RightParentheses)?;
                ast::Type::Unit
            }
            other => bail!("Expected type token, got {other:?}"),
        })
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

        let return_ty = match self.peek_token() {
            Some(Token::RightArrow) => {
                let _ = self.eat_token();
                self.parse_type()?
            }
            Some(Token::LeftBrace) => Type::Unit,
            Some(other) => bail!("Expected function return type, got {other:?} instead"),
            None => bail!("Expected function return type, reached end of token stream instead"),
        };

        let sym_ref = self.sym.add_function(Function::new(name, return_ty))?;

        let compound = self.parse_compound()?;

        Ok(ast::FunctionDefinition {
            compound,
            sym: sym_ref,
        })
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
            if !self.has_tokens() {
                break;
            }

            functions.push(self.parse_function_def()?);
        }

        Ok(ast::Root { functions })
    }
}
