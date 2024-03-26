use core::cell::RefCell;

use crate::{error::CompilerError, warning::CompilerWarning};

/// Handler that contains information to be emitted.
/// Uses `RefCell` around the `HandlerInner` to avoid excessive `&mut`
#[derive(Default, Debug, Clone)]
pub struct Handler {
    inner: RefCell<HandlerInner>,
}

/// Inner struct that contains actual data
#[derive(Default, Debug, Clone)]
struct HandlerInner {
    errors: Vec<CompilerError>,
    warnings: Vec<CompilerWarning>,
}

impl Handler {
    pub fn emit_err(&self, err: CompilerError) -> ErrorEmitted {
        self.inner.borrow_mut().errors.push(err);
        ErrorEmitted::emit()
    }

    pub fn emit_warn(&self, warn: CompilerWarning) {
        self.inner.borrow_mut().warnings.push(warn)
    }

    pub fn get_inner(self) -> (Vec<CompilerError>, Vec<CompilerWarning>) {
        let inner = self.inner.into_inner();
        (inner.errors, inner.warnings)
    }
}

/// Dummy struct to prove that an error occurred and was emitted.
/// Returned in place of some `CompilerError` (i.e., `LexError`, `ParserError`, `TypeError`, etc.).
#[derive(Debug)]
pub struct ErrorEmitted {
    _phantom: (),
}

impl ErrorEmitted {
    pub fn emit() -> Self {
        Self { _phantom: () }
    }
}
