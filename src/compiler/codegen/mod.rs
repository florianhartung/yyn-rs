use std::path::Path;

use anyhow::{bail, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};

use crate::compiler::parser::ast;
use crate::compiler::parser::ast::{CompoundExpr, Expr, FunctionDefinition, Root, Type};

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
        let return_ty = self.return_ty.as_llvm_type(codegen);

        let param_types = &[];
        let is_var_args = false;

        let fn_ty = return_ty.map_or_else(
            || {
                codegen
                    .context
                    .void_type()
                    .fn_type(param_types, is_var_args)
            },
            |ty| ty.fn_type(param_types, is_var_args),
        );
        let fn_value = codegen.module.add_function(&self.name, fn_ty, None);

        let block = codegen.context.append_basic_block(fn_value, "fn_start");
        codegen.builder.position_at_end(block);
        let actual_ret_ty = self.compound.codegen(codegen)?;

        if actual_ret_ty != return_ty {
            bail!(
                "Expected function '{}' to return type '{:?}', but it actually returns a '{:?}'",
                &self.name,
                return_ty,
                actual_ret_ty,
            );
        }

        Ok(())
    }
}

impl CompoundExpr {
    /// Returns the type of value returned by this, `None` if this compound does not contain a return statement
    fn codegen<'ctx>(self, codegen: &CodeGen<'ctx>) -> Result<Option<BasicTypeEnum<'ctx>>> {
        for e in self.expressions {
            match e {
                Expr::Return(num) => return Ok(Some(generate_return_expression(codegen, num)?)),
                other => other.codegen(codegen)?,
            }
        }
        Ok(None)
    }
}

fn generate_return_expression<'ctx>(
    codegen: &CodeGen<'ctx>,
    value: u32,
) -> Result<BasicTypeEnum<'ctx>> {
    let return_ty = codegen.context.i32_type();
    let return_val = return_ty.const_int(value as u64, false);
    codegen.builder.build_return(Some(&return_val))?;
    Ok(return_ty.into())
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
            Expr::Return(_) => {
                unimplemented!("return expression is not handled in this function")
            }
        }
    }
}

impl ast::Type {
    fn as_llvm_type<'ctx>(&self, codegen: &CodeGen<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
        match self {
            Type::Int => Some(codegen.context.i32_type().into()),
            Type::Unit => None,
        }
    }
}
