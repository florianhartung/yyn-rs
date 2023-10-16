use anyhow::Result;

use crate::compiler::ast_analyzer::symbol_table::{Function, SymbolTable};
use crate::compiler::parser::ast;

pub mod symbol_table;

pub fn analyze_ast(ast: ast::Root) -> Result<AnalyzedAST> {
    let mut table = SymbolTable::new();

    for f in &ast.functions {
        table.add_function(f.name.clone(), Function::new(f.return_ty))?;
    }

    Ok(AnalyzedAST { ast, table })
}

pub struct AnalyzedAST {
    pub ast: ast::Root,
    pub table: SymbolTable,
}
