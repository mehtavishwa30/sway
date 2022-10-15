use sway_types::{Ident, Span};

use crate::{
    language::{parsed, typed::*},
    transform,
};

pub(crate) struct TypedAbiDeclaration {
    pub(crate) name: Ident,
    pub(crate) interface_surface: Vec<TypedFunctionDeclaration>,
    pub(crate) methods: Vec<parsed::FunctionDeclaration>,
    pub(crate) span: Span,
    pub(crate) attributes: transform::AttributesMap,
}
