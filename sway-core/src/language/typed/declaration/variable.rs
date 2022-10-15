use crate::{
    language::{ty::VariableMutability, typed::*},
    type_system::ResolvedType,
};
use sway_types::{Ident, Span};

pub(crate) struct TypedVariableDeclaration {
    pub(crate) mutability: VariableMutability,
    pub(crate) name: Ident,
    pub(crate) type_ascription: ResolvedType,
    pub(crate) body: TypedExpression,
    pub(crate) type_ascription_span: Option<Span>,
}
