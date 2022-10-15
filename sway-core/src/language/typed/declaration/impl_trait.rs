use sway_types::Span;

use crate::{
    language::{typed::*, CallPath},
    type_system::ResolvedType,
};

pub(crate) struct TypedImplTrait {
    pub(crate) trait_name: CallPath,
    pub(crate) type_implementing_for: ResolvedType,
    pub(crate) type_implementing_for_span: Span,
    pub(crate) methods: Vec<TypedFunctionDeclaration>,
    pub(crate) span: Span,
}
