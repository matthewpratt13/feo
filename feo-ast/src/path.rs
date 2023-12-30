use std::str::FromStr;

use feo_error::{handler, type_error::TypeErrorKind};
use feo_types::span::{Span, Spanned};

use crate::{identifier::Identifier, punctuation::PuncKind};

#[derive(Debug, Clone)]
pub struct PathTypeSegment {
    pub identifier: Identifier,
}

impl PathTypeSegment {
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }
}

#[derive(Debug, Clone)]
pub struct PathType {
    pub prefix: PathTypeSegment,
    pub suffix: Vec<(PuncKind, PathTypeSegment)>,
    span: Span,
}

impl PathType {
    pub fn tokenize(
        src: &str,
        start: usize,
        prefix: Identifier,
        segments: Vec<(&str, &str)>,
        handler: &mut Handler,
    ) -> Result<Option<Token>, ErrorEmitted> {
        let mut suffix: Vec<(PuncKind, PathTypeSegment)> = Vec::new();

        let mut i = start;

        for s in segments {
            let double_colon = PuncKind::from_str(s.0).map_err(|_| {
                handler.emit_err(CompileError::Type(TypeErrorKind::InvalidPathSeparator))
            })?;

            // skip double colon
            i += s.0.len();

            let span_ = Span::new(src, i, i + s.1.len());

            let identifier_ = Identifier::new(s.1.to_string(), span_);

            let segment = PathTypeSegment::new(identifier_);

            suffix.push((double_colon, segment));

            i += s.1.len()
        }

        let span = Span::new(src, start, i);

        let path = Self {
            prefix,
            suffix,
            span,
        };

        let token = Token::Path(path);

        Ok(Some(token))
    }
}

impl Spanned for PathType {
    fn span(&self) -> &Span {
        &self.span
    }
}
