use std::env;
use std::path::PathBuf;
use std::process::exit;

mod compiler;

fn main() {
    let Some(src_file) = env::args().skip(1).next() else {
        println!("Please specify a path to a file containing yyn source code");
        exit(1);
    };

    let src_file = PathBuf::from(src_file);
    let out_file = src_file.with_extension("ll");

    compiler::compile(&src_file, &out_file).expect("Failed compilation");
}
