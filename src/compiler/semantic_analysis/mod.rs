use anyhow::Result;

use crate::compiler::parser::ast;
use crate::compiler::symbol_table::Sym;

pub fn analyse(ast: ast::Root, _sym: Sym) -> Result<AnalyzedAST> {
    // TODO type checking
    Ok(AnalyzedAST { ast })
}

pub struct AnalyzedAST {
    pub ast: ast::Root,
}
