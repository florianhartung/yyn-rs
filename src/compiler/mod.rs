use crate::compiler::lexer::Lexer;

mod token;
mod lexer;

// TODO add compiler options/flags
#[derive(Debug)] // is this necessary?
pub struct YYNCompiler {}

impl YYNCompiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&self, src_code: &str) -> String {
        let lexer = Lexer::new(src_code);
        let tokens = lexer.tokenize().unwrap();

        // print tokens for now
        println!("{tokens:?}");

        todo!("Parse tokens into AST & generate LLVM from AST");
    }
}