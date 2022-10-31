#![allow(dead_code)]
use crate::core::token::{AstToken, Token, TypedAstToken};
use sway_core::{
    declaration_engine,
    language::{parsed::Declaration, ty},
    transform,
};
use sway_types::Spanned;

pub(crate) fn attributes_map2(token: &Token) -> Option<transform::AttributesMap> {
    match &token.typed.as_ref()? {
        TypedAstToken::TypedDeclaration(decl) => match decl {
            ty::TyDeclaration::EnumDeclaration(decl_id) => Some(
                declaration_engine::de_get_enum(decl_id.clone(), &decl.span())
                    .ok()?
                    .attributes,
            ),
            ty::TyDeclaration::FunctionDeclaration(decl_id) => Some(
                declaration_engine::de_get_function(decl_id.clone(), &decl.span())
                    .ok()?
                    .attributes,
            ),
            ty::TyDeclaration::StructDeclaration(decl_id) => Some(
                declaration_engine::de_get_struct(decl_id.clone(), &decl.span())
                    .ok()?
                    .attributes,
            ),
            ty::TyDeclaration::ConstantDeclaration(decl_id) => Some(
                declaration_engine::de_get_constant(decl_id.clone(), &decl.span())
                    .ok()?
                    .attributes,
            ),
            ty::TyDeclaration::StorageDeclaration(decl_id) => Some(
                declaration_engine::de_get_storage(decl_id.clone(), &decl.span())
                    .ok()?
                    .attributes,
            ),
            _ => None,
        },
        TypedAstToken::TypedStorageField(field) => Some(field.attributes.clone()),
        TypedAstToken::TypedStructField(field) => Some(field.attributes.clone()),
        TypedAstToken::TypedTraitFn(trait_fn) => Some(trait_fn.attributes.clone()),
        TypedAstToken::TypedEnumVariant(variant) => Some(variant.attributes.clone()),
        _ => None,
    }
}

pub(crate) fn attributes_map(token: &Token) -> Option<&transform::AttributesMap> {
    match &token.parsed {
        AstToken::Declaration(declaration) => match declaration {
            Declaration::EnumDeclaration(decl) => Some(&decl.attributes),
            Declaration::FunctionDeclaration(decl) => Some(&decl.attributes),
            Declaration::StructDeclaration(decl) => Some(&decl.attributes),
            Declaration::ConstantDeclaration(decl) => Some(&decl.attributes),
            Declaration::StorageDeclaration(decl) => Some(&decl.attributes),
            _ => None,
        },
        AstToken::StorageField(field) => Some(&field.attributes),
        AstToken::StructField(field) => Some(&field.attributes),
        AstToken::TraitFn(trait_fn) => Some(&trait_fn.attributes),
        AstToken::EnumVariant(variant) => Some(&variant.attributes),
        _ => None,
    }
}

pub(crate) fn doc_attributes(token: &Token) -> Option<&[transform::Attribute]> {
    attributes_map(token)
        .and_then(|attributes| attributes.get(&transform::AttributeKind::Doc))
        .map(Vec::as_slice)
}

pub(crate) fn storage_attributes(token: &Token) -> Option<&[transform::Attribute]> {
    attributes_map(token)
        .and_then(|attributes| attributes.get(&transform::AttributeKind::Storage))
        .map(Vec::as_slice)
}
