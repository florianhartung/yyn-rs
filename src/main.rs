use anyhow::Result;
use std::env;
use std::path::PathBuf;
use std::process::exit;

use anyhow::Context;

mod compiler;

fn main() -> Result<()> {
    // First argument is program name, it can be skipped
    let mut args = env::args().skip(1);

    let Some(src_file): Option<PathBuf> = args.next().map(PathBuf::from).into() else {
        println!("Please specify a path to a file containing yyn source code");
        exit(1);
    };

    let dest_file = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| src_file.with_extension("ll"));

    compiler::compile(&src_file, &dest_file).context("Failed compilation")
}
