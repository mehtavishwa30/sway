use crate::{
    language::{typed::*, Purity, Visibility},
    transform,
    type_system::{ResolvedType, TypeInfo},
};
use sway_types::{Ident, Span};

pub(crate) struct TypedFunctionDeclaration {
    pub(crate) visibility: Visibility,
    pub(crate) name: Ident,
    pub(crate) type_parameters: Vec<ResolvedTypeParameter>,
    pub(crate) parameters: Vec<TypedFunctionParameter>,
    pub(crate) resolved_return_type: ResolvedType,
    pub(crate) return_span: Span,
    pub(crate) body: TypedCodeBlock,
    pub(crate) span: Span,
    pub(crate) attributes: transform::AttributesMap,
    pub(crate) initial_type_info: TypeInfo,
    pub(crate) is_contract_call: bool,
    pub(crate) purity: Purity,
}

pub(crate) struct TypedFunctionParameter {
    pub(crate) name: Ident,
    pub(crate) is_reference: bool,
    pub(crate) is_mutable: bool,
    pub(crate) mutability_span: Span,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) type_span: Span,
    pub(crate) initial_type_info: TypeInfo,
}
