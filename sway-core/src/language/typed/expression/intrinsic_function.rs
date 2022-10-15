use crate::language::typed::*;
use sway_ast::Intrinsic;
use sway_types::Span;

pub(crate) struct TypedIntrinsicFunctionKind {
    pub(crate) kind: Intrinsic,
    pub(crate) arguments: Vec<TypedExpression>,
    pub(crate) type_arguments: Vec<ResolvedTypeArgument>,
    pub(crate) span: Span,
}
