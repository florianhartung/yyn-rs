use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::compiler::parser::ast;

pub struct SymbolTable {
    top_level_functions: HashMap<String, Function>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            top_level_functions: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, f: Function) -> Result<()> {
        if self.top_level_functions.contains_key(&name) {
            bail!("Redefinition of function `{name}`");
        }
        self.top_level_functions.insert(name, f);

        Ok(())
    }

    pub fn get_function(&self, name: &str) -> Option<&Function> {
        self.top_level_functions.get(name)
    }
}

pub struct Function {
    pub return_ty: ast::Type,
}

impl Function {
    pub fn new(return_ty: ast::Type) -> Self {
        Self { return_ty }
    }
}
