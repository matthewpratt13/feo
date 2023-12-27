use std::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};

use feo_types::{
    error::{TypeError, TypeErrorKind},
    span::Span,
};
use feo_types::{PuncKind, Punctuation};

use crate::token::{Token, Tokenize};

impl Tokenize for Punctuation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedPunctuation,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let punc_kind = PuncKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
    }
}
