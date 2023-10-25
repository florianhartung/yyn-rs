use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use crate::compiler::ref_arena::{Arena, ArenaRef};

pub mod function;
pub use function::*;

/// A symbol table containing information about all functions.
/// This is a wrapper type for `InnerSym` which actually contains all the data.
/// This is needed so its data can be immutably referenced from the AST and new entries can be added to it at the same time.
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
}

struct InnerSym {
    functions: Arena<Function>,
    function_name_lookup: HashMap<String, ArenaRef<Function>>,
}

impl InnerSym {
    fn new() -> Self {
        Self {
            functions: Arena::new(),
            function_name_lookup: HashMap::new(),
        }
    }
}
