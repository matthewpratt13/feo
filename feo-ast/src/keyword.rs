use std::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};

use feo_types::{
    error::{TypeError, TypeErrorKind},
    span::Span,
};
use feo_types::{Keyword, KeywordKind};

use crate::token::{Token, Tokenize};

impl Tokenize for Keyword {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let err = TypeError {
            error_kind: TypeErrorKind::UnrecognizedKeyword,
            pos: start,
        };

        // convert `TypeErrorKind` to `CompileError::Type(TypeError)`
        let keyword_kind = KeywordKind::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Type(err)))?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}
