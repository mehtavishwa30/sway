use sway_types::{state::StateIndex, Ident, Span};

use crate::{
    language::{ty, typed::*},
    ResolvedType,
};

pub(crate) struct TypedReassignment {
    pub(crate) lhs_base_name: Ident,
    pub(crate) lhs_type: ResolvedType,
    pub(crate) lhs_indices: Vec<ty::ProjectionKind>,
    pub(crate) rhs: TypedExpression,
}

pub(crate) struct TypedStorageReassignment {
    pub(crate) fields: Vec<TypedStorageReassignDescriptor>,
    pub(crate) ix: StateIndex,
    pub(crate) rhs: TypedExpression,
}

pub(crate) struct TypedStorageReassignDescriptor {
    pub(crate) name: Ident,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) span: Span,
}
