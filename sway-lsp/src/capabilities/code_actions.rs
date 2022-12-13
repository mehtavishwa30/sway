pub mod abi_impl;

use crate::core::{session::Session, token::TypedAstToken};
pub use crate::error::DocumentError;
use abi_impl::abi_impl_code_action;
use std::sync::Arc;
use sway_core::{declaration_engine, language::ty::TyDeclaration};
use sway_types::Spanned;
use tower_lsp::lsp_types::{CodeActionResponse, Range, TextDocumentIdentifier, Url};

pub(crate) fn code_actions(
    session: Arc<Session>,
    range: &Range,
    text_document: TextDocumentIdentifier,
    temp_uri: &Url,
) -> Option<CodeActionResponse> {
    let (_, token) = session.token_at_position(temp_uri, range.start.clone())?;
    token.typed.and_then(|typed_token| {
        let maybe_decl = match typed_token {
            TypedAstToken::TypedDeclaration(decl) => Some(decl),
            _ => None,
        };

        maybe_decl
            .and_then(|decl| match decl {
                TyDeclaration::AbiDeclaration(ref decl_id) => Some(declaration_engine::de_get_abi(
                    decl_id.clone(),
                    &decl_id.span(),
                )),
                // Add code actions for other declaration types here
                _ => None,
            })
            .and_then(|result| {
                result.ok().and_then(|abi_decl| {
                    Some(vec![abi_impl_code_action(abi_decl, text_document.uri)])
                })
            })
    })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         core::document::TextDocument,
//         test_utils::{get_absolute_path, get_url},
//     };
//     use serial_test::serial;

//     use tower_lsp::lsp_types::{Position, Range, TextDocumentIdentifier};

//     #[test]
//     fn code_actions_returns_none() {
//         let session = Arc::new(Session::new());
//         let range = Range {
//             start: Position {
//                 character: 0,
//                 line: 0,
//             },
//             end: Position {
//                 character: 0,
//                 line: 0,
//             },
//         };

//         let uri = get_url(&get_absolute_path("sway-lsp/test/fixtures/cats.txt"));
//         let text_document = TextDocumentIdentifier { uri: uri.clone() };

//         let result = code_actions(session, &range, text_document, &uri);
//         assert!(result.is_none(), "result = {:?}", result);
//     }

//     #[tokio::test]
//     #[serial]
//     async fn code_actions_returns_abi_impl() {
//         let session = Session::new();
//         let path = get_absolute_path("sway-lsp/test/fixtures/fizzbuzz/src/main.sw");
//         let document = TextDocument::build_from_path(&path.clone()).unwrap();
//         let _ = Session::store_document(&session, document);

//         let range = Range {
//             start: Position {
//                 character: 5,
//                 line: 10,
//             },
//             end: Position {
//                 character: 5,
//                 line: 10,
//             },
//         };

//         let main_uri = get_url(&path);
//         let text_document_id = TextDocumentIdentifier {
//             uri: main_uri.clone(),
//         };
//         let _ = session.init(&get_url(&get_absolute_path(
//             "sway-lsp/test/fixtures/fizzbuzz/Forc.toml",
//         )));

//         eprintln!("\n\n\n\nsession.token_map {:?}", session.token_map());

//         let result = code_actions(Arc::new(session), &range, text_document_id, &main_uri);
//         assert!(result.is_some(), "result = {:?}", result);
//     }
// }
