use crate::core::token::{Token, TypeDefinition};
use sway_core::{
    declaration_engine::{de_get_abi, de_get_generic_declaration, get_generic_declaration},
    type_system, TypeInfo,
};
use sway_types::{Ident, Span, Spanned, TypeId};
use tower_lsp::lsp_types::{Position, Range};

/// Given a cursor `Position`, return the `Ident` of a token in the
/// Iterator if one exists at that position.
pub(crate) fn ident_at_position<I>(cursor_position: Position, tokens: I) -> Option<Ident>
where
    I: Iterator<Item = (Ident, Token)>,
{
    for (ident, _) in tokens {
        // eprintln!("identt {:?}", ident.clone());
        let range = get_range_from_span(&ident.span());
        // eprintln!("range.startt {:?}", range.start.clone());
        // eprintln!("range.endd {:?}", range.end.clone());
        if cursor_position >= range.start && cursor_position <= range.end {
            return Some(ident);
        }
    }
    None
}

/// Given a cursor `Position`, return the `TypeId` of a token in the
/// Iterator if one exists at that position.
pub(crate) fn type_id_at_position<I>(cursor_position: Position, tokens: I) -> Option<TypeId>
where
    I: Iterator<Item = (TypeId, Token)>,
{
    for (type_id, _) in tokens {
        // eprintln!("identt {:?}", ident.clone());

        let range = get_range_from_span(&ident.span());
        // eprintln!("range.startt {:?}", range.start.clone());
        // eprintln!("range.endd {:?}", range.end.clone());
        if cursor_position >= range.start && cursor_position <= range.end {
            return Some(ident);
        }
    }
    None
}

/// Given a `Span`, convert into an `lsp_types::Range` and return.
pub(crate) fn get_range_from_span(span: &Span) -> Range {
    let start = span.start_pos().line_col();
    let end = span.end_pos().line_col();

    let start_line = start.0 as u32 - 1;
    let start_character = start.1 as u32 - 1;

    let end_line = end.0 as u32 - 1;
    let end_character = end.1 as u32 - 1;

    Range {
        start: Position::new(start_line, start_character),
        end: Position::new(end_line, end_character),
    }
}

/// Given a `TypeDefinition`, convert into an `lsp_types::Range` and return.
pub(crate) fn get_range_from_type_def(type_def: &TypeDefinition) -> Range {
    let span = match type_def {
        TypeDefinition::TypeId(type_id) => {
            de_get_generic_declaration(**type_id).unwrap().span();
        }
        TypeDefinition::Ident(ident) => ident.span(),
    };
    get_range_from_span(&span)
}
