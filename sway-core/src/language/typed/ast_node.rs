use crate::language::typed::*;
use sway_types::Span;

pub(crate) struct TypedAstNode {
    pub(crate) content: TypedAstNodeContent,
    pub(crate) span: Span,
}

pub(crate) enum TypedAstNodeContent {
    Declaration(TypedDeclaration),
    Expression(TypedExpression),
    ImplicitReturnExpression(TypedExpression),
    SideEffect,
}
