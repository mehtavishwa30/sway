use sway_types::{state::StateIndex, Ident, Span};

use crate::type_system::ResolvedType;

pub(crate) struct TypedStorageAccess {
    pub(crate) fields: Vec<TypedStorageAccessDescriptor>,
    pub(crate) ix: StateIndex,
}

pub(crate) struct TypedStorageAccessDescriptor {
    pub(crate) name: Ident,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) span: Span,
}
