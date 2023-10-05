mod compiler;

fn main() {
    let compiler = YYNCompiler::new(true);
    let asm_code = compiler.compile("fun main() {\nhello\n}");
    println!("--- ASM ---\n{}", asm_code);
}
