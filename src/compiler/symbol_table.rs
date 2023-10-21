use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use anyhow::{bail, Result};
use generational_arena::Arena;

use crate::compiler::codegen::CodegenFunctionDataRef;
use crate::compiler::parser::ast;

/// A symbol table containing information about all functions.
/// This is a wrapper type for `InnerSym` which actually contains all the data.
/// This is needed so its data can be referenced from the AST and new entries can be added to it at the same time.
#[derive(Clone)]
pub struct Sym {
    inner: Rc<RefCell<InnerSym>>,
}
impl Sym {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(InnerSym::new())),
        }
    }

    /// Returns the function from a FunctionRef. This can guarantee a function to be found.
    pub fn get_function(&self, fn_ref: FunctionRef) -> impl Deref<Target = Function> + '_ {
        Ref::map(self.inner.borrow(), |inner_sym| {
            inner_sym.get_function(fn_ref)
        })
    }

    /// Returns the function from a FunctionRef. This can guarantee a function to be found.
    pub fn get_function_mut(&self, fn_ref: FunctionRef) -> impl DerefMut<Target = Function> + '_ {
        RefMut::map(self.inner.borrow_mut(), |inner_sym| {
            inner_sym.get_function_mut(fn_ref)
        })
    }

    /// Tries to find a function with specified name. Returns `None` if no function was found.
    pub fn get_function_by_name<'a>(
        &'a self,
        name: &str,
    ) -> Option<impl Deref<Target = Function> + 'a> {
        Ref::filter_map(self.inner.borrow(), |inner_sym| {
            inner_sym.get_function_by_name(name)
        })
        .ok()
    }

    /// Adds a new function to the symbol table. This can fail if a function is already defined.
    pub fn add_function(&self, f: Function) -> Result<FunctionRef> {
        self.inner.borrow_mut().add_function(f)
    }
}

struct InnerSym {
    functions: Arena<Function>,
    function_name_lookup: HashMap<String, generational_arena::Index>,
}

impl InnerSym {
    fn new() -> Self {
        Self {
            functions: Arena::new(),
            function_name_lookup: HashMap::new(),
        }
    }

    fn add_function(&mut self, f: Function) -> Result<FunctionRef> {
        if self.function_name_lookup.contains_key(&f.name) {
            bail!("Redefinition of function `{}`", f.name);
        }

        let name = f.name.clone();

        let idx = self.functions.insert(f);
        self.function_name_lookup.insert(name, idx);

        Ok(FunctionRef(idx))
    }

    fn get_function_by_name<'a>(&'a self, name: &'_ str) -> Option<&'a Function> {
        self.function_name_lookup
            .get(name)
            .map(|&idx| self.get_function(FunctionRef(idx)))
    }

    fn get_function(&self, fn_ref: FunctionRef) -> &Function {
        self.functions
            .get(fn_ref.0)
            .expect("the index to be valid because it came from the lookup table")
    }

    fn get_function_mut(&mut self, fn_ref: FunctionRef) -> &mut Function {
        self.functions
            .get_mut(fn_ref.0)
            .expect("the index to be valid because it came from the lookup table")
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub return_ty: ast::Type,
    pub codegen_data_ref: Option<CodegenFunctionDataRef>,
}

impl Function {
    pub fn new(name: String, return_ty: ast::Type) -> Self {
        Self {
            name,
            return_ty,
            codegen_data_ref: None,
        }
    }

    /// Fails if codegen data is already attached
    pub fn attach_codegen_data(&mut self, llvm_ty: CodegenFunctionDataRef) -> Result<()> {
        if self.codegen_data_ref.is_some() {
            bail!("Failed to attach new codegen data to {self:?}. It already has data attached to it.")
        }
        self.codegen_data_ref = Some(llvm_ty);

        Ok(())
    }
}

/// A reference to a function inside an arena.
/// This reference can only be created by inserting a function into the symbol table.
/// As long as the symbol table is alive, this reference can be used to get the associated function back with [Sym::get_function].
#[derive(Copy, Clone, Debug)]
pub struct FunctionRef(generational_arena::Index);
