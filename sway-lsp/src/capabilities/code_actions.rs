pub use crate::error::DocumentError;
use crate::{
    core::session::Session,
    utils::token::{abi_declaration_of_type_id, type_id_of_token},
};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionResponse, Position, Range,
    TextDocumentIdentifier, TextEdit, Url, WorkspaceEdit,
};

fn get_contract_impl_string(contract_name: String, function_names: Vec<String>) -> String {
    let mut result = format!(r#"impl {} for Contract {{"#, contract_name);
    for function_name in function_names.iter() {
        result.push_str(&format!("\n    fn {}() {{}}", function_name));
    }
    result.push_str(&"\n}");
    result
}

//

pub(crate) fn code_actions(
    session: Arc<Session>,
    range: &Range,
    text_document: TextDocumentIdentifier,
    temp_uri: &Url,
) -> Option<CodeActionResponse> {
    // Check if highlighted portion contains an ABI
    // get contractName
    // get function signatures
    // construct impl block
    eprintln!("\n\n\n\n\n\n\n\nOh nooooo !");

    eprintln!("range.start.clone() {:?}", range.start.clone());

    let (_, token) = session.token_at_position(temp_uri, range.start.clone())?;

    eprintln!("token {:?}", token);

    let type_id1 = type_id_of_token(token.clone());
    eprintln!("type_id1 {:?}", type_id1);

    type_id_of_token(token)
        .and_then(|type_id| {
            eprintln!("{:?}", type_id);
            abi_declaration_of_type_id(&type_id, session.token_map())
        })
        .and_then(|abi_decl| {
            eprintln!("{:?}", abi_decl);

            let contract_name = "MyyyyContract";
            let function_name = "test_function";

            let text_edit = TextEdit {
                range: Range {
                    start: Position {
                        line: range.start.line + 2,
                        character: 0,
                    }, // TODO: last line of file? last line of impl + 1?
                    end: Position {
                        line: range.end.line + 2,
                        character: 0,
                    },
                },
                new_text: get_contract_impl_string(
                    contract_name.to_string(),
                    vec![function_name.to_string()],
                ),
            };
            let mut text_edit_map = HashMap::new();
            text_edit_map.insert(text_document.uri.clone(), vec![text_edit]);

            return Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
                title: String::from("Generate impl for contract"),
                kind: Some(CodeActionKind::REFACTOR),
                edit: Some(WorkspaceEdit {
                    changes: Some(text_edit_map),
                    change_annotations: None, // TODO: default default
                    document_changes: None,
                }),
                // edit: None,
                diagnostics: None,
                command: None,
                data: Some(Value::String(text_document.uri.to_string())),
                is_preferred: Some(false),
                disabled: None,
            })]);
        });

    return None;
}

// todo: tests
