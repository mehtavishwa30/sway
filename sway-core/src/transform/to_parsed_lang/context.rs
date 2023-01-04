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
    module_has_configurable_block: bool,
}

impl Context {
    /// docstring 
    pub fn set_module_has_configurable_block(&self) {
        self.inner.borrow_mut().module_has_configurable_block = true;
    }

    /// docstring
    pub fn module_has_configurable_block(&self) -> bool {
        self.inner.borrow().module_has_configurable_block
    }
}
