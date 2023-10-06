#[derive(Debug)]
pub struct Root {
    pub(crate) function_names: Vec<String>,
    pub(crate) functions: Vec<FunctionDefinition>,
}

#[derive(Debug)]
pub struct CompoundExpr {
    pub(crate) expressions: Vec<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Compound(Vec<Box<Expr>>),
    Exit(u32),
    FnCall(String),
}

#[derive(Debug)]
pub struct FunctionDefinition {
    pub(crate) name: String,
    pub(crate) compound: CompoundExpr,
}
