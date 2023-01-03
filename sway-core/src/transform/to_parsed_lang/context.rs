use core::cell::RefCell;

/// A context containing global information used during `convert_parse_tree"
#[derive(Default)]
pub struct Context {
    /// The inner context.
    /// This construction is used to avoid `&mut` all over `convert_parse_tree`.
    inner: RefCell<ContextInner>,
}

/// Contains the actual data for `Context`.
/// Modelled this way to afford an API using interior mutability.
#[derive(Default)]
struct ContextInner {
    /// docstring
    found_configurable_block: bool,
}

impl Context {
    /// docstring
    pub fn has_configurable_block(self) -> bool {
        self.inner.into_inner().found_configurable_block
    }
}
