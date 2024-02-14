use feo_ast::{expression::AttributeKind, path::SimplePathSegmentKind};
use feo_types::{keyword::KeywordKind, Keyword};

use crate::{parse::Peek, parser::Peeker};

impl Peek for AttributeKind {
    fn peek(peeker: Peeker<'_>) -> Option<Self>
    where
        Self: Sized,
    {
        let attr_kind = if let Some(k) = Keyword::peek(peeker) {
            match k.keyword_kind {
                KeywordKind::KwAbstract => AttributeKind::KwAbstract(k),
                KeywordKind::KwExport => AttributeKind::KwExport(k),
                KeywordKind::KwExtern => AttributeKind::KwExtern(k),
                KeywordKind::KwUnsafe => AttributeKind::KwUnsafe(k),
                _ => return None,
            }
        } else if let Some(p) = SimplePathSegmentKind::peek(peeker) {
            AttributeKind::Path(p)
        } else {
            return None;
        };

        Some(attr_kind)
    }
}
