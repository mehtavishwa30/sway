use sway_error::error::CompileError;
use sway_types::Spanned;

use crate::{
    error::*,
    language::{parsed::*, ty},
    semantic_analysis::*,
    type_system::*,
};

impl ty::TyStructDeclaration {
    pub(crate) fn type_check(
        ctx: TypeCheckContext,
        decl: StructDeclaration,
    ) -> CompileResult<Self> {
        let mut warnings = vec![];
        let mut errors = vec![];

        let type_engine = ctx.type_engine;
        let declaration_engine = ctx.declaration_engine;
        let engines = ctx.engines();

        let StructDeclaration {
            name,
            fields,
            type_parameters,
            visibility,
            span,
            attributes,
            ..
        } = decl;

        // create a namespace for the decl, used to create a scope for generics
        let mut decl_namespace = ctx.namespace.clone();
        let mut ctx = ctx.scoped(&mut decl_namespace);

        // type check the type parameters
        // insert them into the namespace
        let mut new_type_parameters = vec![];
        for type_parameter in type_parameters.into_iter() {
            if !type_parameter.trait_constraints.is_empty() {
                errors.push(CompileError::WhereClauseNotYetSupported {
                    span: type_parameter.trait_constraints_span,
                });
                return err(warnings, errors);
            }
            new_type_parameters.push(check!(
                TypeParameter::type_check(ctx.by_ref(), type_parameter),
                return err(warnings, errors),
                warnings,
                errors
            ));
        }

        // Create the type param for the self type and add it to the struct
        // params. This will also add it to the namespace.
        let self_type_param = TypeParameter::new_self_type_param(ctx.by_ref(), &name.span());
        let self_type_id = self_type_param.type_id;
        new_type_parameters.push(self_type_param);

        // type check the fields
        let mut new_fields = vec![];
        for field in fields.into_iter() {
            new_fields.push(check!(
                ty::TyStructField::type_check(ctx.by_ref(), field),
                return err(warnings, errors),
                warnings,
                errors
            ));
        }

        // create the struct decl
        let decl = ty::TyStructDeclaration {
            name: name.clone(),
            type_parameters: new_type_parameters,
            fields: new_fields,
            visibility,
            span,
            attributes,
        };

        // Unify the type of the new struct with the self type param.
        check!(
            CompileResult::from(type_engine.unify(
                declaration_engine,
                decl.create_type_id(engines),
                self_type_id,
                &name.span(),
                "self type",
                None
            )),
            return err(warnings, errors),
            warnings,
            errors
        );

        ok(decl, warnings, errors)
    }
}

impl ty::TyStructField {
    pub(crate) fn type_check(mut ctx: TypeCheckContext, field: StructField) -> CompileResult<Self> {
        let mut warnings = vec![];
        let mut errors = vec![];
        let type_engine = ctx.type_engine;
        let declaration_engine = ctx.declaration_engine;
        let initial_type_id = type_engine.insert_type(declaration_engine, field.type_info);
        let r#type = check!(
            ctx.resolve_type(
                initial_type_id,
                &field.type_span,
                EnforceTypeArguments::Yes,
                None
            ),
            type_engine.insert_type(declaration_engine, TypeInfo::ErrorRecovery),
            warnings,
            errors,
        );
        let field = ty::TyStructField {
            name: field.name,
            type_id: r#type,
            initial_type_id,
            span: field.span,
            type_span: field.type_span,
            attributes: field.attributes,
        };
        ok(field, warnings, errors)
    }
}
