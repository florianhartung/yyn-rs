use std::cell::Ref;
use std::ops::Deref;

use anyhow::bail;
use anyhow::Result;

use crate::compiler::parser::ast;
use crate::compiler::ref_arena::ArenaRef;
use crate::compiler::symbol_table::{InnerSym, Sym};

#[derive(Debug)]
pub struct Function {
    /// name must be unique
    pub name: String,
    pub return_ty: ast::Type,
}

impl Function {
    pub fn new(name: String, return_ty: ast::Type) -> Self {
        Self { name, return_ty }
    }
}
impl Sym {
    /// Tries to find a function with specified name. Returns `None` if no function was found.
    pub fn get_function_by_name<'a>(
        &'a self,
        name: &str,
    ) -> Option<impl Deref<Target = ArenaRef<Function>> + 'a> {
        Ref::filter_map(self.inner.borrow(), |inner_sym| {
            inner_sym.get_function_by_name(name)
        })
        .ok()
    }

    /// Adds a new function to the symbol table. This can fail if a function is already defined.
    pub fn add_function(&self, f: Function) -> Result<ArenaRef<Function>> {
        self.inner.borrow_mut().add_function(f)
    }
}

impl InnerSym {
    fn add_function(&mut self, f: Function) -> Result<ArenaRef<Function>> {
        if self.function_name_lookup.contains_key(&f.name) {
            bail!("Redefinition of function `{}`", f.name);
        }

        let name = f.name.clone();

        let fn_ref = self.functions.insert(f);

        self.function_name_lookup.insert(name, fn_ref.clone());

        Ok(fn_ref)
    }

    fn get_function_by_name<'a>(&'a self, name: &'_ str) -> Option<&'a ArenaRef<Function>> {
        self.function_name_lookup.get(name)
    }
}
