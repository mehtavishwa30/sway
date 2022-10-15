use sway_types::Ident;

use crate::{
    language::{parsed, typed::*, Visibility},
    transform,
};

pub(crate) struct TypedTraitDeclaration {
    pub(crate) visibility: Visibility,
    pub(crate) name: Ident,
    pub(crate) supertraits: Vec<parsed::Supertrait>,
    pub(crate) interface_surface: Vec<TypedFunctionDeclaration>,
    pub(crate) methods: Vec<parsed::FunctionDeclaration>,
    pub(crate) attributes: transform::AttributesMap,
}
