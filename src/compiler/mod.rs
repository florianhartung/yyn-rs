use std::arch::asm;
use std::collections::VecDeque;

use anyhow::Result;
use crate::compiler::cg::nasm::NasmCodegen;

use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;

mod lexer;
mod parser;
mod cg;

// TODO add compiler options/flags
#[derive(Debug)] // is this necessary?
pub struct YYNCompiler {}

impl YYNCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, src_code: &str) -> Result<String> {
        let lexer = Lexer::new(src_code);
        let tokens = lexer.tokenize()?;

        println!("----------- Tokens -----------");
        println!("{tokens:?}");

        let parser = Parser::new(VecDeque::from(tokens));
        let ast = parser.parse_root()?;

        println!("------------ AST -------------");
        println!("{ast:?}");

        let codegen = NasmCodegen::new(ast);
        let asm_code = codegen.generate()?;

        println!("---------- Assembly ----------");
        println!("{asm_code}");

        Ok(asm_code)
    }
}