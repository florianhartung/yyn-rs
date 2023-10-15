use std::path::Path;

use anyhow::Result;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::BasicMetadataValueEnum;

use crate::compiler::parser::ast::{CompoundExpr, Expr, FunctionDefinition, Root};

pub fn generate(ast_root: Root, llvm_ir_out: &Path) -> Result<()> {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("main_module");

    let codegen = CodeGen {
        context: &context,
        builder,
        module,
    };

    ast_root.codegen(&codegen)?;

    eprintln!("--- LLVM IR ---");
    codegen.module.print_to_stderr();
    codegen.module.write_bitcode_to_path(llvm_ir_out);

    Ok(())
}

struct CodeGen<'a> {
    context: &'a Context,
    builder: Builder<'a>,
    module: Module<'a>,
}

impl Root {
    fn codegen(self, codegen: &CodeGen) -> Result<()> {
        self.functions
            .into_iter()
            .map(|func| func.codegen(codegen))
            .collect()
    }
}

impl FunctionDefinition {
    fn codegen(self, codegen: &CodeGen) -> Result<()> {
        let fn_ty = codegen.context.void_type().fn_type(&[], false);
        let fn_value = codegen.module.add_function(&self.name, fn_ty, None);
        let block = codegen
            .context
            .append_basic_block(fn_value, "function_start");
        codegen.builder.position_at_end(block);

        self.compound.codegen(codegen)?;

        codegen.builder.build_return(None)?;

        Ok(())
    }
}

impl CompoundExpr {
    fn codegen(self, codegen: &CodeGen) -> Result<()> {
        self.expressions
            .into_iter()
            .map(|e| e.codegen(codegen))
            .collect()
    }
}

impl Expr {
    fn codegen(self, codegen: &CodeGen) -> Result<()> {
        match self {
            Expr::Compound(child_expressions) => child_expressions
                .expressions
                .into_iter()
                .map(|c| c.codegen(codegen))
                .collect(),
            Expr::Exit(exit_code) => {
                let void_ty = codegen.context.void_type();
                let i_ty = codegen.context.i32_type();
                let fn_type = void_ty.fn_type(&[BasicMetadataTypeEnum::from(i_ty)], false);

                let fn_val =
                    codegen
                        .module
                        .add_function("ExitProcess@4", fn_type, Some(Linkage::External));

                codegen.builder.build_call(
                    fn_val,
                    &[i_ty.const_int(exit_code as u64, false).into()],
                    "tmpexitprocess",
                )?;

                Ok(())
            }
            Expr::FnCall(_) => {
                todo!("call function")
            }
        }
    }
}
