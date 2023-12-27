use std::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};

use feo_types::{
    error::{TypeError, TypeErrorKind},
    span::Span,
};
use feo_types::{DelimKind, DelimOrientation, Delimiter};

use crate::token::{Token, Tokenize};

impl Tokenize for Delimiter {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedDelimiter,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let delim_kind = DelimKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err.clone())))?;

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let delim_orientation = DelimOrientation::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let delim = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delim);

        Ok(Some(token))
    }
}
