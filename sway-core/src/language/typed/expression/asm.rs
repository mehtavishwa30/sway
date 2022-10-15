use crate::language::typed::*;
use sway_types::Ident;

pub(crate) struct TypedAsmRegisterDeclaration {
    pub(crate) name: Ident,
    pub(crate) initializer: Option<TypedExpression>,
}
