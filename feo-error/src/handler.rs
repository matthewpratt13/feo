use core::cell::RefCell;

use crate::error::CompilerError;
use crate::warning::CompilerWarning;

#[derive(Default, Debug, Clone)]
pub struct Handler {
    inner: RefCell<HandlerInner>,
}

#[derive(Default, Debug, Clone)]
struct HandlerInner {
    errors: Vec<CompilerError>,
    warnings: Vec<CompilerWarning>,
}

impl Handler {
    pub fn emit_err(&mut self, err: CompilerError) -> ErrorEmitted {
        self.inner.borrow_mut().errors.push(err);
        ErrorEmitted { _private: () }
    }

    pub fn emit_warn(&mut self, warn: CompilerWarning) {
        self.inner.borrow_mut().warnings.push(warn)
    }

    pub fn get(self) -> (Vec<CompilerError>, Vec<CompilerWarning>) {
        let inner = self.inner.into_inner();
        (inner.errors, inner.warnings)
    }
}

impl From<(Vec<CompilerError>, Vec<CompilerWarning>)> for Handler {
    fn from(value: (Vec<CompilerError>, Vec<CompilerWarning>)) -> Self {
        Self {
            inner: RefCell::new(HandlerInner {
                errors: value.0,
                warnings: value.1,
            }),
        }
    }
}
// dummy struct to prove that an error occurred and was emitted
#[derive(Debug)]
pub struct ErrorEmitted {
    _private: (),
}
