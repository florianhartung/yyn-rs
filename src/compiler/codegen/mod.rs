use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;

use anyhow::{anyhow, Result};
use anyhow::{bail, Context as AnyhowContext};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::{Linkage, Module};
use inkwell::types::{BasicMetadataTypeEnum, BasicType};
use inkwell::values::FunctionValue;

use crate::compiler::codegen::types::{CompoundReturnType, Type};
use crate::compiler::parser::ast::{CompoundExpr, Expr, FunctionDefinition, Root};
use crate::compiler::ref_arena::ArenaRef;
use crate::compiler::semantic_analysis::AnalyzedAST;
use crate::compiler::symbol_table::Function;
use crate::compiler::symbol_table::Sym;

mod types;

pub fn generate(ast_root: AnalyzedAST, sym: Sym, llvm_ir_out: &Path) -> Result<()> {
    let context = Rc::new(Context::create());

    let builder = context.create_builder();
    let module = context.create_module("main_module");

    let mut codegen = CodegenContext {
        context: &context,
        builder,
        module,
        sym,
        functions: HashMap::new(),
    };

    ast_root.ast.codegen(&mut codegen)?;

    eprintln!("--- LLVM IR ---");
    codegen.module.print_to_stderr();
    codegen.module.write_bitcode_to_path(llvm_ir_out);

    Ok(())
}

/// Context for code generation
pub struct CodegenContext<'cx> {
    context: &'cx Context,
    builder: Builder<'cx>,
    module: Module<'cx>,
    sym: Sym,
    functions: HashMap<String, FunctionValue<'cx>>,
}

impl<'cx> CodegenContext<'cx> {
    /// Fails if codegen data is already attached
    pub fn generate_function_value<'ctx>(
        &mut self,
        fn_sym: &ArenaRef<Function>,
    ) -> Result<FunctionValue> {
        // Get information from symbol table
        let fn_sym = fn_sym.get();

        // Check if function value is generated already
        if self.functions.get(&fn_sym.name).is_some() {
            bail!("Failed to attach new function value to function {}. It already has data attached to it.", fn_sym.name);
        }

        // Generate function value
        let return_ty = Type::from_ast_type(&fn_sym.return_ty, self);
        let fn_ty = return_ty.fn_type(&[], false);
        let fn_value = self.module.add_function(&fn_sym.name, fn_ty, None);

        self.functions.insert(fn_sym.name.clone(), fn_value);

        Ok(fn_value)
    }
}

impl Root {
    fn codegen(self, codegen: &mut CodegenContext) -> Result<()> {
        // First generate all LLVM function types so they are already available when building call expressions later
        // They are collected into the filed `codegen.functions`
        for f in &self.functions {
            codegen.generate_function_value(&f.sym)?;
        }

        self.functions
            .into_iter()
            .map(|func| func.codegen(codegen))
            .collect()
    }
}

impl FunctionDefinition {
    fn codegen(self, codegen: &CodegenContext) -> Result<()> {
        let fn_sym = self.sym.get();

        let Some(fn_value) = codegen.functions.get(&fn_sym.name) else {
            panic!("LLVM Function value is not generated yet, but should have been. Lazy function value generation is not supported yet")
        };

        let block = codegen.context.append_basic_block(*fn_value, "");
        codegen.builder.position_at_end(block);

        let actual_ret_ty = self.compound.codegen(codegen)?.into_type(codegen);
        codegen.builder.clear_insertion_position();

        if actual_ret_ty.to_basic_type_enum() != fn_value.get_type().get_return_type() {
            bail!(
                "Expected function '{}' to return type '{:?}', but it actually returns a '{:?}'",
                fn_sym.name,
                fn_value.get_type().get_return_type(),
                actual_ret_ty,
            );
        }

        Ok(())
    }
}

impl CompoundExpr {
    /// Returns the type of value returned by this compound
    fn codegen<'ctx>(self, codegen: &CodegenContext<'ctx>) -> Result<CompoundReturnType<'ctx>> {
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

fn generate_explicit_return<'ctx>(
    codegen: &CodegenContext<'ctx>,
    value: u32,
) -> Result<Type<'ctx>> {
    let return_ty = codegen.context.i32_type();
    let return_val = return_ty.const_int(value as u64, false);
    codegen.builder.build_return(Some(&return_val))?;

    Ok(Type::BasicType(return_ty.as_basic_type_enum()))
}

impl Expr {
    fn codegen(self, codegen: &CodegenContext) -> Result<()> {
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

impl Function {}
