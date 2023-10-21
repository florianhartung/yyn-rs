use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, VoidType};

use crate::compiler::codegen::CodegenContext;
use crate::compiler::parser::ast;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Type<'ctx> {
    BasicType(BasicTypeEnum<'ctx>),
    Void(VoidType<'ctx>),
}

impl<'ctx> Type<'ctx> {
    pub fn fn_type(
        &self,
        param_types: &[BasicMetadataTypeEnum<'ctx>],
        is_var_args: bool,
    ) -> FunctionType<'ctx> {
        match self {
            Type::Void(ty) => ty.fn_type(param_types, is_var_args),
            Type::BasicType(ty) => ty.fn_type(param_types, is_var_args),
        }
    }

    pub fn to_basic_type_enum(&self) -> Option<BasicTypeEnum<'ctx>> {
        match self {
            Type::BasicType(ty) => Some(*ty),
            Type::Void(_) => None,
        }
    }

    pub fn from_ast_type<'a, 'b>(
        ast_ty: &'a ast::Type,
        codegen: &'b CodegenContext<'ctx>,
    ) -> Type<'ctx> {
        match ast_ty {
            ast::Type::Unit => Self::Void(codegen.context.void_type()),
            ast::Type::Int => Self::BasicType(codegen.context.i32_type().as_basic_type_enum()),
        }
    }
}

impl ast::Type {
    pub fn as_llvm_type<'ctx>(
        &self,
        codegen: &CodegenContext<'ctx>,
    ) -> Option<BasicTypeEnum<'ctx>> {
        match self {
            ast::Type::Int => Some(codegen.context.i32_type().into()),
            ast::Type::Unit => None,
        }
    }
}

pub enum CompoundReturnType<'ctx> {
    Explicit(Type<'ctx>),
    ImplicitUnit,
}

impl<'ctx> CompoundReturnType<'ctx> {
    pub fn into_type(self, codegen: &CodegenContext<'ctx>) -> Type<'ctx> {
        match self {
            CompoundReturnType::Explicit(ty) => ty,
            CompoundReturnType::ImplicitUnit => Type::Void(codegen.context.void_type()),
        }
    }
}
