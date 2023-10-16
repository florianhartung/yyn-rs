use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType, VoidType};

use crate::compiler::codegen::Cx;
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

    pub fn from_ast_type(ast_ty: &ast::Type, codegen: &Cx<'ctx>) -> Self {
        match ast_ty {
            ast::Type::Unit => Self::Void(codegen.context.void_type()),
            ast::Type::Int => Self::BasicType(codegen.context.i32_type().as_basic_type_enum()),
        }
    }
}

impl ast::Type {
    fn as_llvm_type<'ctx>(&self, codegen: &Cx<'ctx>) -> Option<BasicTypeEnum<'ctx>> {
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
    pub fn into_type(self, codegen: &Cx<'ctx>) -> Type<'ctx> {
        match self {
            CompoundReturnType::Explicit(ty) => ty,
            CompoundReturnType::ImplicitUnit => Type::Void(codegen.context.void_type()),
        }
    }
}
