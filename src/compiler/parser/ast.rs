#[derive(Debug)]
pub struct Root {
    pub(crate) function_names: Vec<String>,
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
    pub(crate) name: String,
    pub(crate) compound: CompoundExpr,
    pub(crate) return_ty: Type,
}

#[derive(Debug)]
pub enum Type {
    Unit,
    Int,
}
