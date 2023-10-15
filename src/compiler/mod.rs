use std::fs::File;
use std::path::Path;

use anyhow::{Context, Result};

mod codegen;
mod lexer;
mod parser;

// TODO add compiler options/flags

pub fn compile(src: &Path, llvm_ir_out: &Path) -> Result<()> {
    let src_code = std::fs::read_to_string(src).context("Failed to read source code file")?;

    let tokens = lexer::tokenize(&src_code)?;
    dbg!(&tokens);

    let ast_root = parser::parse(tokens)?;
    dbg!(&ast_root);

    codegen::generate(ast_root, &llvm_ir_out)
}
