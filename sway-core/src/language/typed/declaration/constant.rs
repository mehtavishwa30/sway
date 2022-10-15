use sway_types::Ident;

use crate::{
    language::{typed::*, Visibility},
    transform,
};

pub(crate) struct TypedConstantDeclaration {
    pub(crate) name: Ident,
    pub(crate) value: TypedExpression,
    pub(crate) visibility: Visibility,
    pub(crate) attributes: transform::AttributesMap,
}
