use crate::compiler::YYNCompiler;

mod compiler;

fn main() {
    let compiler = YYNCompiler::new();
    let asm_code = compiler.compile(include_str!("../programs/exitwithcode.yyn"));
    println!("--- ASM ---\n{}", asm_code);
}
