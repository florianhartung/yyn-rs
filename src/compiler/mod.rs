use anyhow::Result;

mod lexer;
mod parser;
mod codegen;

// TODO add compiler options/flags

pub fn compile(src_code: &str) -> Result<()> {
    let tokens = lexer::tokenize(src_code)?;
    dbg!(&tokens);

    let ast_root = parser::parse(tokens)?;
    dbg!(&ast_root);

    let _ = codegen::generate(&ast_root)?;

    todo!("emit LLVM IR or OBJ code")
}