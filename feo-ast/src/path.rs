#![allow(dead_code)]

use crate::{identifier::Identifier, item::DblColon, keyword::KeywordKind};

pub enum PathSegment {
    Identifier(Identifier),
    CrateKeyword(KeywordKind),
    SelfKeyword(KeywordKind),
    SuperKeyword(KeywordKind),
}

pub struct SimplePath {
    dbl_colon_opt: Option<DblColon>,
    first_segment: PathSegment,
    subsequent_segments: Vec<(DblColon, PathSegment)>,
}
