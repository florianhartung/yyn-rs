use std::collections::HashMap;
use std::path::Path;

use anyhow::Context as AnyhowContext;
use anyhow::{anyhow, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::FunctionValue;

use crate::compiler::ast_analyzer::symbol_table::SymbolTable;
use crate::compiler::ast_analyzer::AnalyzedAST;
use crate::compiler::codegen::types::{CompoundReturnType, Type};
use crate::compiler::parser::ast::{CompoundExpr, Expr, FunctionDefinition, Root};

mod types;

pub fn generate(ast_root: AnalyzedAST, llvm_ir_out: &Path) -> Result<()> {
    let context = Context::create();

    let builder = context.create_builder();
    let module = context.create_module("main_module");

    let mut codegen = Cx {
        context: &context,
        builder,
        module,
        table: ast_root.table,
        functions: HashMap::new(),
    };

    ast_root.ast.codegen(&mut codegen)?;

    eprintln!("--- LLVM IR ---");
    codegen.module.print_to_stderr();
    codegen.module.write_bitcode_to_path(llvm_ir_out);

    Ok(())
}

/// Context for code generation
pub struct Cx<'cx> {
    context: &'cx Context,
    builder: Builder<'cx>,
    module: Module<'cx>,
    table: SymbolTable,
    functions: HashMap<String, FunctionValue<'cx>>,
}

impl Root {
    fn codegen(self, codegen: &mut Cx) -> Result<()> {
        // First generate all function types
        for f in &self.functions {
            let fn_value = f.generate_fn_value(codegen)?;
            codegen.functions.insert(f.name.clone(), fn_value);
        }

        self.functions
            .into_iter()
            .map(|func| func.codegen(codegen))
            .collect()
    }
}

impl FunctionDefinition {
    fn generate_fn_value<'cx>(&self, codegen: &Cx<'cx>) -> Result<FunctionValue<'cx>> {
        let return_ty = Type::from_ast_type(&self.return_ty, codegen);
        let fn_ty = return_ty.fn_type(&[], false);
        let fn_value = codegen.module.add_function(&self.name, fn_ty, None);

        Ok(fn_value)
    }

    fn codegen(self, codegen: &Cx) -> Result<()> {
        let fn_value = codegen
            .functions
            .get(&self.name)
            .expect("function value to exist");
        let block = codegen.context.append_basic_block(*fn_value, "");
        codegen.builder.position_at_end(block);
        let actual_ret_ty = self.compound.codegen(codegen)?.into_type(codegen);
        codegen.builder.clear_insertion_position();

        // if actual_ret_ty != fn_value.get_type().get_return_type() {
        //     bail!(
        //         "Expected function '{}' to return type '{:?}', but it actually returns a '{:?}'",
        //         &self.name,
        //         return_ty,
        //         actual_ret_ty,
        //     );
        // }

        Ok(())
    }
}

impl CompoundExpr {
    /// Returns the type of value returned by this compound
    fn codegen<'ctx>(self, codegen: &Cx<'ctx>) -> Result<CompoundReturnType<'ctx>> {
        for e in self.expressions {
            match e {
                // If there is a return statement, cancel code generation of this compound, generate the return statement and return the type of the return value
                Expr::Return(num) => {
                    return Ok(CompoundReturnType::Explicit(generate_explicit_return(
                        codegen, num,
                    )?));
                }
                // If an inner compound explicitly returns a value, we stop generating code for this compound and return the type return value
                Expr::Compound(inner_compound) => {
                    if let CompoundReturnType::Explicit(ty) = inner_compound.codegen(codegen)? {
                        return Ok(CompoundReturnType::Explicit(ty));
                    }
                }
                // The code for all other expressions can simply be generated, as it does not affect control flow at a function level
                other => other.codegen(codegen)?,
            }
        }

        codegen.builder.build_return(None)?;
        Ok(CompoundReturnType::ImplicitUnit)
    }
}

fn generate_explicit_return<'ctx>(codegen: &Cx<'ctx>, value: u32) -> Result<Type<'ctx>> {
    let return_ty = codegen.context.i32_type();
    let return_val = return_ty.const_int(value as u64, false);
    codegen.builder.build_return(Some(&return_val))?;

    Ok(Type::BasicType(return_ty.as_basic_type_enum()))
}

impl Expr {
    fn codegen(self, codegen: &Cx) -> Result<()> {
        match self {
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
            Expr::FnCall(name) => {
                let fn_value = codegen
                    .module
                    .get_function(&name)
                    .context(anyhow!("Unknown function `{name}` referenced"))?;

                codegen.builder.build_call(fn_value, &[], "call_fn")?;
                Ok(())
            }
            Expr::Return(_) => {
                unimplemented!("return expression is not handled in this function")
            }
            Expr::Compound(_) => {
                unimplemented!("compound is not be handled in this function")
            }
        }
    }
}
