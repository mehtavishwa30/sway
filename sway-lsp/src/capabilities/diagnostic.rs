use crate::core::token::Token;
use sway_error::error::CompileError;
use sway_error::warning::{CompileWarning, Warning};
use sway_types::{Ident, LineCol, Spanned};
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn get_diagnostics<I>(
    mut tokens: I,
    warnings: &[CompileWarning],
    errors: &[CompileError],
) -> Vec<Diagnostic>
where
    I: Iterator<Item = (Ident, Token)>,
{
    let errors = errors.iter().map(|error| Diagnostic {
        range: get_range(error.span().line_col()),
        severity: Some(DiagnosticSeverity::ERROR),
        message: format!("{}", error),
        ..Default::default()
    });

    let warnings: Vec<_> = warnings
        .iter()
        //.filter(|warning| warning.warning_content != Warning::DeadFunctionDeclaration)
        // maybe write a method that just returns the function declaration tokens from the file
                        // so we only need to loop over those instead of the whole map! thanks mitch 
        .map(|warning| {
            let span = if warning.warning_content != Warning::DeadFunctionDeclaration {
                warning.span()
            } else {
                tokens.find(|(ident, token)| {
                    ident.span() == warning.span
                }).map(|(ident, token)| ident.span()).unwrap()
            };
            
            Diagnostic {
            range: get_range(warning.span().line_col()),
            severity: Some(DiagnosticSeverity::WARNING),
            message: warning.to_friendly_warning_string(),
            ..Default::default()
        }})
        .collect();

    let mut all = Vec::with_capacity(errors.len() + warnings.len());
    all.extend(errors);
    all.extend(warnings);
    all
}

fn get_range((start, end): (LineCol, LineCol)) -> Range {
    let pos = |lc: LineCol| Position::new(lc.line as u32 - 1, lc.col as u32 - 1);
    let start = pos(start);
    let end = pos(end);
    Range { start, end }
}
