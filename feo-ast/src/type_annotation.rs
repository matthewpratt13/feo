use std::str::FromStr;
use std::sync::Arc;

use feo_error::error::{CompileError, ErrorEmitted};
use feo_types::{span::Span, TypeAnnotation, TypeName};

use crate::token::{Token, Tokenize};

impl Tokenize for TypeAnnotation {
    fn tokenize(
        src: &Arc<&str>,
        content: &str,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let span = Span::new(src, start, end);

        let type_name = TypeName::from_str(content)
            .map_err(|_| ErrorEmitted::emit_err(CompileError::Infallible))?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}
