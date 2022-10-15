use sway_types::Span;

use crate::type_system::{ResolvedType, TypeInfo};

pub(crate) struct ResolvedTypeArgument {
    pub(crate) resolved_type: ResolvedType,
    pub(crate) span: Span,
    pub(crate) initial_type_info: TypeInfo,
}
