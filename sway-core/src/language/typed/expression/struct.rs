use sway_types::Ident;

use crate::language::typed::*;

pub(crate) struct TypedStructExpressionField {
    pub(crate) name: Ident,
    pub(crate) value: TypedExpression,
}
