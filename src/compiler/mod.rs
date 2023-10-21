use std::path::Path;

use anyhow::{Context, Result};

use crate::compiler::symbol_table::Sym;

mod codegen;
mod lexer;
mod parser;
mod semantic_analysis;
pub mod symbol_table;

// TODO add compiler options/flags

pub fn compile(src: &Path, llvm_ir_out: &Path) -> Result<()> {
    let src_code = std::fs::read_to_string(src).context("Failed to read source code file")?;

    let tokens = lexer::tokenize(&src_code)?;
    dbg!(&tokens);

    let sym = Sym::new();

    let ast_root = parser::parse(tokens, sym.clone())?;
    dbg!(&ast_root);

    let analyzed_ast_root = semantic_analysis::analyse(ast_root, sym.clone())?;

    codegen::generate(analyzed_ast_root, sym.clone(), llvm_ir_out)
}
