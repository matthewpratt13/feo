use std::error::Error;
use std::fmt;

use feo_types::{error::TypeErrorKind, span::Position};

#[derive(Default, Debug, Clone)]
pub struct TypeError {
    pub error_kind: TypeErrorKind,
    pub position: Position,
}

impl Error for TypeError {}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}:{}",
            self.error_kind,
            self.position.line_col().0,
            self.position.line_col().1
        )
    }
}
