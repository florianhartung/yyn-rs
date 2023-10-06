use crate::compiler::YYNCompiler;

mod compiler;

fn main() {
    let compiler = YYNCompiler::new();
    let asm_code = compiler.compile(include_str!("../programs/functions.yyn"))
        .expect("Failed compilation");
    std::fs::write("out.nasm", asm_code).expect("file write to succeed");
}
