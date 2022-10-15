use crate::{language::typed::*, transform, type_system::ResolvedType};
use sway_types::{Ident, Span};

pub(crate) struct TypedStorageDeclaration {
    pub(crate) fields: Vec<TypedStorageField>,
    pub(crate) span: Span,
    pub(crate) attributes: transform::AttributesMap,
}

pub(crate) struct TypedStorageField {
    pub(crate) name: Ident,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) resolved_type_span: Span,
    pub(crate) initializer: TypedExpression,
    pub(crate) span: Span,
    pub(crate) attributes: transform::AttributesMap,
}
