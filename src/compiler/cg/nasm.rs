use anyhow::bail;
use anyhow::Result;
use itertools::Itertools;

use crate::compiler::parser::ast;
use crate::compiler::parser::ast::Expr;

pub struct NasmCodegen {
    ast_root: ast::Root,
    asm_code: String,
}

impl NasmCodegen {
    pub fn new(ast_root: ast::Root) -> Self {
        Self {
            ast_root,
            asm_code: String::new(),
        }
    }

    fn tab(&mut self) {
        self.asm_code.push('\t');
    }

    fn newline(&mut self) {
        self.asm_code.push('\n');
    }

    /// Add instruction
    fn inst<const T: usize>(&mut self, instruction: &str, args: [&str; T]) {
        self.tab();
        self.asm_code.push_str(instruction);
        self.tab();
        let combined_args = args.into_iter().intersperse(", ").collect::<String>();
        self.asm_code.push_str(&combined_args);
        self.newline();
    }

    fn label(&mut self, label: &str) {
        self.asm_code.push_str(label);
        self.asm_code.push(':');
        self.newline();
    }

    fn generate_exit(&mut self, exit_code: u32) {
        self.inst("push", [&exit_code.to_string()]);
        self.inst("call", ["_ExitProcess@4"]);
    }

    fn generate_fn_call(&mut self, fn_name: &str) {
        self.inst("call", [fn_name]);
    }

    fn generate_fn(&mut self, function_def: ast::FunctionDefinition) -> Result<()> {
        let label_name = match function_def.name.as_str() {
            "main" => "_main",
            x => x,
        };
        self.label(label_name);

        for expr in function_def.compound.expressions {
            match expr {
                Expr::Exit(exit_code) => {
                    self.generate_exit(exit_code)
                }
                Expr::FnCall(fn_name) => {
                    if !self.ast_root.function_names.contains(&fn_name) {
                        bail!("Trying to call non-existent function `{fn_name}. Consider defining this function.");
                    }
                    self.generate_fn_call(&fn_name)
                }
                _ => bail!("nested compounds are not supported yet"),
            }
        }

        self.inst("ret", []);

        Ok(())
    }

    pub fn generate(mut self) -> Result<String> {
        self.asm_code.push_str("section .text\n");
        self.asm_code.push_str("global _main\n");
        self.asm_code.push_str("extern  _ExitProcess@4\n");

        let functions = std::mem::take(&mut self.ast_root.functions);
        for function_def in functions {
            self.generate_fn(function_def)?;
        }

        Ok(self.asm_code)
    }
}
