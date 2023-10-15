mod compiler;

fn main() {
    compiler::compile(include_str!("../programs/functions.yyn")).expect("Failed compilation");
}
