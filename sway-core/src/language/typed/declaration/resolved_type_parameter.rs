use sway_types::Ident;

use crate::{
    language::CallPath,
    type_system::{ResolvedType, TypeInfo},
};

pub(crate) struct ResolvedTypeParameter {
    pub(crate) name_ident: Ident,
    pub(crate) resolved_type: ResolvedType,
    pub(crate) trait_constraints: Vec<ResolvedTraitConstraint>,
    pub(crate) initial_type_info: TypeInfo,
}

pub(crate) struct ResolvedTraitConstraint {
    pub(crate) call_path: CallPath,
}
