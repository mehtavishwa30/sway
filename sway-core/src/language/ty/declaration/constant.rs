use sway_types::{Ident, Span, Spanned};

use crate::{
    language::{ty::*, Visibility},
    transform,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyConstantDeclaration {
    pub name: Ident,
    pub value: TyExpression,
    pub visibility: Visibility,
    pub attributes: transform::AttributesMap,
    pub span: Span,
}

impl Spanned for TyConstantDeclaration {
    fn span(&self) -> Span {
        self.span.clone()
    }
}
