use std::{
    fmt::{self, Debug},
    hash::Hasher,
};

use sway_types::{Ident, Span};

use crate::{
    decl_engine::*,
    engine_threading::*,
    error::*,
    language::{parsed, ty::*},
    transform::AttributeKind,
    type_system::*,
    types::DeterministicallyAborts,
};

pub trait GetDeclIdent {
    fn get_decl_ident(&self, decl_engine: &DeclEngine) -> Option<Ident>;
}

#[derive(Clone, Debug)]
pub struct TyAstNode {
    pub content: TyAstNodeContent,
    pub(crate) span: Span,
}

// NOTE: Hash and PartialEq must uphold the invariant:
// k1 == k2 -> hash(k1) == hash(k2)
// https://doc.rust-lang.org/std/collections/struct.HashMap.html
impl EqWithEngines for TyAstNode {}
impl PartialEqWithEngines for TyAstNode {
    fn eq(&self, other: &Self, engines: Engines<'_>) -> bool {
        self.content.eq(&other.content, engines)
    }
}

impl HashWithEngines for TyAstNode {
    fn hash<H: Hasher>(&self, state: &mut H, type_engine: &TypeEngine) {
        self.content.hash(state, type_engine);
    }
}

impl DisplayWithEngines for TyAstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>, engines: Engines<'_>) -> fmt::Result {
        use TyAstNodeContent::*;
        match &self.content {
            Declaration(typed_decl) => DisplayWithEngines::fmt(typed_decl, f, engines),
            Expression(exp) => DisplayWithEngines::fmt(exp, f, engines),
            ImplicitReturnExpression(exp) => write!(f, "return {}", engines.help_out(exp)),
            SideEffect => f.write_str(""),
        }
    }
}

impl SubstTypes for TyAstNode {
    fn subst_inner(&mut self, type_mapping: &TypeSubstMap, engines: Engines<'_>) {
        match self.content {
            TyAstNodeContent::ImplicitReturnExpression(ref mut exp) => {
                exp.subst(type_mapping, engines)
            }
            TyAstNodeContent::Declaration(ref mut decl) => decl.subst(type_mapping, engines),
            TyAstNodeContent::Expression(ref mut expr) => expr.subst(type_mapping, engines),
            TyAstNodeContent::SideEffect => (),
        }
    }
}

impl ReplaceSelfType for TyAstNode {
    fn replace_self_type(&mut self, engines: Engines<'_>, self_type: TypeId) {
        match self.content {
            TyAstNodeContent::ImplicitReturnExpression(ref mut exp) => {
                exp.replace_self_type(engines, self_type)
            }
            TyAstNodeContent::Declaration(ref mut decl) => {
                decl.replace_self_type(engines, self_type)
            }
            TyAstNodeContent::Expression(ref mut expr) => {
                expr.replace_self_type(engines, self_type)
            }
            TyAstNodeContent::SideEffect => (),
        }
    }
}

impl ReplaceDecls for TyAstNode {
    fn replace_decls_inner(&mut self, decl_mapping: &DeclMapping, engines: Engines<'_>) {
        match self.content {
            TyAstNodeContent::ImplicitReturnExpression(ref mut exp) => {
                exp.replace_decls(decl_mapping, engines)
            }
            TyAstNodeContent::Declaration(_) => {}
            TyAstNodeContent::Expression(ref mut expr) => expr.replace_decls(decl_mapping, engines),
            TyAstNodeContent::SideEffect => (),
        }
    }
}

impl CollectTypesMetadata for TyAstNode {
    fn collect_types_metadata(
        &self,
        ctx: &mut CollectTypesMetadataContext,
    ) -> CompileResult<Vec<TypeMetadata>> {
        self.content.collect_types_metadata(ctx)
    }
}

impl DeterministicallyAborts for TyAstNode {
    fn deterministically_aborts(&self, decl_engine: &DeclEngine, check_call_body: bool) -> bool {
        use TyAstNodeContent::*;
        match &self.content {
            Declaration(_) => false,
            Expression(exp) | ImplicitReturnExpression(exp) => {
                exp.deterministically_aborts(decl_engine, check_call_body)
            }
            SideEffect => false,
        }
    }
}

impl GetDeclIdent for TyAstNode {
    fn get_decl_ident(&self, decl_engine: &DeclEngine) -> Option<Ident> {
        self.content.get_decl_ident(decl_engine)
    }
}

impl TyAstNode {
    /// recurse into `self` and get any return statements -- used to validate that all returns
    /// do indeed return the correct type
    /// This does _not_ extract implicit return statements as those are not control flow! This is
    /// _only_ for explicit returns.
    pub(crate) fn gather_return_statements(&self) -> Vec<&TyExpression> {
        match &self.content {
            TyAstNodeContent::ImplicitReturnExpression(ref exp) => exp.gather_return_statements(),
            // assignments and  reassignments can happen during control flow and can abort
            TyAstNodeContent::Declaration(TyDeclaration::VariableDeclaration(decl)) => {
                decl.body.gather_return_statements()
            }
            TyAstNodeContent::Expression(exp) => exp.gather_return_statements(),
            TyAstNodeContent::SideEffect | TyAstNodeContent::Declaration(_) => vec![],
        }
    }

    /// Returns `true` if this AST node will be exported in a library, i.e. it is a public declaration.
    pub(crate) fn is_public(&self, decl_engine: &DeclEngine) -> CompileResult<bool> {
        let mut warnings = vec![];
        let mut errors = vec![];
        let public = match &self.content {
            TyAstNodeContent::Declaration(decl) => {
                let visibility = check!(
                    decl.visibility(decl_engine),
                    return err(warnings, errors),
                    warnings,
                    errors
                );
                visibility.is_public()
            }
            TyAstNodeContent::Expression(_)
            | TyAstNodeContent::SideEffect
            | TyAstNodeContent::ImplicitReturnExpression(_) => false,
        };
        ok(public, warnings, errors)
    }

    /// Naive check to see if this node is a function declaration of a function called `main` if
    /// the [TreeType] is Script or Predicate.
    pub(crate) fn is_main_function(
        &self,
        decl_engine: &DeclEngine,
        tree_type: parsed::TreeType,
    ) -> CompileResult<bool> {
        let mut warnings = vec![];
        let mut errors = vec![];
        match &self {
            TyAstNode {
                span,
                content: TyAstNodeContent::Declaration(TyDeclaration::FunctionDeclaration(decl_id)),
                ..
            } => {
                let TyFunctionDeclaration { name, .. } = check!(
                    CompileResult::from(decl_engine.get_function(decl_id.clone(), span)),
                    return err(warnings, errors),
                    warnings,
                    errors
                );
                let is_main = name.as_str() == sway_types::constants::DEFAULT_ENTRY_POINT_FN_NAME
                    && matches!(
                        tree_type,
                        parsed::TreeType::Script | parsed::TreeType::Predicate
                    );
                ok(is_main, warnings, errors)
            }
            _ => ok(false, warnings, errors),
        }
    }

    /// Check to see if this node is a function declaration of a function annotated as test.
    pub(crate) fn is_test_function(&self, decl_engine: &DeclEngine) -> CompileResult<bool> {
        let mut warnings = vec![];
        let mut errors = vec![];
        match &self {
            TyAstNode {
                span,
                content: TyAstNodeContent::Declaration(TyDeclaration::FunctionDeclaration(decl_id)),
                ..
            } => {
                let TyFunctionDeclaration { attributes, .. } = check!(
                    CompileResult::from(decl_engine.get_function(decl_id.clone(), span)),
                    return err(warnings, errors),
                    warnings,
                    errors
                );
                ok(
                    attributes.contains_key(&AttributeKind::Test),
                    warnings,
                    errors,
                )
            }
            _ => ok(false, warnings, errors),
        }
    }

    pub(crate) fn type_info(&self, type_engine: &TypeEngine) -> TypeInfo {
        // return statement should be ()
        match &self.content {
            TyAstNodeContent::Declaration(_) => TypeInfo::Tuple(Vec::new()),
            TyAstNodeContent::Expression(TyExpression { return_type, .. }) => {
                type_engine.get(*return_type)
            }
            TyAstNodeContent::ImplicitReturnExpression(TyExpression { return_type, .. }) => {
                type_engine.get(*return_type)
            }
            TyAstNodeContent::SideEffect => TypeInfo::Tuple(Vec::new()),
        }
    }
}

#[derive(Clone, Debug)]
pub enum TyAstNodeContent {
    Declaration(TyDeclaration),
    Expression(TyExpression),
    ImplicitReturnExpression(TyExpression),
    // a no-op node used for something that just issues a side effect, like an import statement.
    SideEffect,
}

impl EqWithEngines for TyAstNodeContent {}
impl PartialEqWithEngines for TyAstNodeContent {
    fn eq(&self, other: &Self, engines: Engines<'_>) -> bool {
        match (self, other) {
            (Self::Declaration(x), Self::Declaration(y)) => x.eq(y, engines),
            (Self::Expression(x), Self::Expression(y)) => x.eq(y, engines),
            (Self::ImplicitReturnExpression(x), Self::ImplicitReturnExpression(y)) => {
                x.eq(y, engines)
            }
            (Self::SideEffect, Self::SideEffect) => true,
            _ => false,
        }
    }
}

impl HashWithEngines for TyAstNodeContent {
    fn hash<H: Hasher>(&self, state: &mut H, type_engine: &TypeEngine) {
        match self {
            TyAstNodeContent::Declaration(decl) => {
                state.write_u8(1);
                decl.hash(state, type_engine);
            }
            TyAstNodeContent::Expression(exp) => {
                state.write_u8(2);
                exp.hash(state, type_engine);
            }
            TyAstNodeContent::ImplicitReturnExpression(exp) => {
                state.write_u8(3);
                exp.hash(state, type_engine);
            }
            TyAstNodeContent::SideEffect => {
                state.write_u8(4);
            }
        }
    }
}

impl CollectTypesMetadata for TyAstNodeContent {
    fn collect_types_metadata(
        &self,
        ctx: &mut CollectTypesMetadataContext,
    ) -> CompileResult<Vec<TypeMetadata>> {
        use TyAstNodeContent::*;
        match self {
            Declaration(decl) => decl.collect_types_metadata(ctx),
            Expression(expr) => expr.collect_types_metadata(ctx),
            ImplicitReturnExpression(expr) => expr.collect_types_metadata(ctx),
            SideEffect => ok(vec![], vec![], vec![]),
        }
    }
}

impl GetDeclIdent for TyAstNodeContent {
    fn get_decl_ident(&self, decl_engine: &DeclEngine) -> Option<Ident> {
        match self {
            TyAstNodeContent::Declaration(decl) => decl.get_decl_ident(decl_engine),
            TyAstNodeContent::Expression(_expr) => None, //expr.get_decl_ident(),
            TyAstNodeContent::ImplicitReturnExpression(_expr) => None, //expr.get_decl_ident(),
            TyAstNodeContent::SideEffect => None,
        }
    }
}
