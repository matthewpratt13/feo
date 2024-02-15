use feo_ast::path::{PathIdenSegmentKind, SimplePathSegmentKind};
use feo_error::parser_error::ParserErrorKind;
use feo_types::keyword::KeywordKind;

use crate::{parse::Peek, parser::Peeker};

impl Peek for SimplePathSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(id) = peeker.peek_identifier() {
            SimplePathSegmentKind::Iden(id)
        } else if let Ok(k) = peeker.peek_keyword() {
            match k.keyword_kind {
                KeywordKind::KwCrate => SimplePathSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => SimplePathSegmentKind::KwSelf(k),
                KeywordKind::KwSuper => SimplePathSegmentKind::KwSuper(k),
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
        } else {
            return Ok(None);
        };

        Ok(Some(segment_kind))
    }
}

impl Peek for PathIdenSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Result<Option<Self>, ParserErrorKind>
    where
        Self: Sized,
    {
        let segment_kind = if let Ok(id) = peeker.peek_identifier() {
            PathIdenSegmentKind::Iden(id)
        } else if let Ok(k) = peeker.peek_keyword() {
            match k.keyword_kind {
                KeywordKind::KwCrate => PathIdenSegmentKind::KwCrate(k),
                KeywordKind::KwSelf => PathIdenSegmentKind::KwSelf(k),
                KeywordKind::KwSelfType => PathIdenSegmentKind::KwSelfType(k),
                KeywordKind::KwSuper => PathIdenSegmentKind::KwSuper(k),
                _ => return Err(ParserErrorKind::UnexpectedToken),
            }
        } else {
            return Ok(None);
        };

        Ok(Some(segment_kind))
    }
}
