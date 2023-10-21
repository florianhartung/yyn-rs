use crate::compiler::symbol_table::FunctionRef;

#[derive(Debug)]
pub struct Root {
    pub(crate) functions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
pub struct CompoundExpr {
    pub expressions: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Compound(Box<CompoundExpr>),
    Exit(u32),
    FnCall(String),
    Return(u32),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub sym: FunctionRef,

    pub compound: CompoundExpr,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Int,
}
