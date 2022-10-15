use crate::language::{ty, typed};

pub(crate) fn transform_to_typed_expression(exp: ty::TyExpression) -> typed::TypedExpression {
    let ty::TyExpression {
        expression,
        return_type,
        span,
    } = exp;

    todo!()
}
