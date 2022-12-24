//! // https://ricardomartins.cc/2016/06/08/interior-mutability

use std::{cell::RefCell, fmt, sync::Arc};

use crate::{engine_threading::*, type_system::*};

pub type TypeRef = Arc<RefCell<TypeInfo>>;

impl DisplayWithEngines for TypeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, engines: Engines<'_>) -> fmt::Result {
        DisplayWithEngines::fmt(&self.into_inner(), f, engines)
    }
}

impl CopyTypes for TypeRef {
    fn copy_types_inner(&mut self, type_mapping: &TypeMapping, engines: Engines<'_>) {
        self.borrow_mut().copy_types(type_mapping, engines);
    }
}

impl ReplaceSelfType for TypeRef {
    fn replace_self_type(&mut self, engines: Engines<'_>, self_type: TypeRef) {
        self.borrow_mut().replace_self_type(engines, self_type);
    }
}

impl CollectTypesMetadata for TypeRef {
    fn collect_types_metadata(
        &self,
        ctx: &mut CollectTypesMetadataContext,
    ) -> CompileResult<Vec<TypeMetadata>> {
        self.borrow().collect_types_metadata(ctx)
    }
}
