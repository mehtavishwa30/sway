use crate::core::session::Session;
pub use crate::error::DocumentError;
use std::{collections::HashMap, sync::Arc};
use tower_lsp::lsp_types::{
    CodeAction, CodeActionKind, CodeActionOrCommand, CodeActionResponse, Position, Range,
    TextDocumentIdentifier, TextEdit, WorkspaceEdit,
};

fn get_contract_impl_string(contract_name: String) -> String {
    format!(
        r#"
impl {} for Contract {{
    fn test_function() {{
        let CONTRACT_B_ID = CONTRACT_B_ID;
        let CONTRACT_C_ID = CONTRACT_C_ID;
    }}
}}
"#,
        contract_name
    )
}

pub(crate) fn code_actions(
    _session: Arc<Session>,
    range: &Range,
    text_document: TextDocumentIdentifier,
) -> Option<CodeActionResponse> {
    // Check if highlighted portion contains an ABI
    // get contractName
    // get function signatures
    // construct impl block
    let is_abi = true; // TODO
    if is_abi == true {
        let contract_name = "MyyyyContract";

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
            new_text: get_contract_impl_string(contract_name.to_string()),
        };
        let mut text_edit_map = HashMap::new();
        text_edit_map.insert(text_document.uri, vec![text_edit]);

        return Some(vec![CodeActionOrCommand::CodeAction(CodeAction {
            title: String::from("Generate impl for contract"),
            kind: Some(CodeActionKind::REFACTOR),
            edit: Some(WorkspaceEdit {
                changes: Some(text_edit_map),
                change_annotations: None, // TODO: default default
                document_changes: None,
            }),
            diagnostics: None,
            command: None,
            data: None,
            is_preferred: Some(false),
            disabled: None,
        })]);
    }
    return None;
}

// todo: tests
