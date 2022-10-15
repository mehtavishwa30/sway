use sway_types::{Ident, Span};

use crate::{
    language::{typed::*, Visibility},
    transform,
    type_system::{ResolvedType, TypeInfo},
};

pub(crate) struct TypedEnumDeclaration {
    pub(crate) visibility: Visibility,
    pub(crate) name: Ident,
    pub(crate) type_parameters: Vec<ResolvedTypeParameter>,
    pub(crate) variants: Vec<TypedEnumVariant>,
    pub(crate) span: Span,
    pub(crate) attributes: transform::AttributesMap,
}

pub(crate) struct TypedEnumVariant {
    pub(crate) name: Ident,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) type_span: Span,
    pub(crate) tag: usize,
    pub(crate) span: Span,
    pub(crate) initial_type_info: TypeInfo,
    pub(crate) attributes: transform::AttributesMap,
}
