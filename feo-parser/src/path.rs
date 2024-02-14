use feo_ast::path::PathIdenSegmentKind;
use feo_types::keyword::KeywordKind;

use crate::{parse::Peek, parser::Peeker};

impl Peek for PathIdenSegmentKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
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
                _ => return None,
            }
        } else {
            return None;
        };

        Some(segment_kind)
    }
}
