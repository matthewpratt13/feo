use crate::{identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind};

pub enum PathSegment {
    IdentifierSeg(Identifier),
    CrateSeg(KeywordKind),
    SelfSeg(KeywordKind),
    SuperSeg(KeywordKind),
}

pub struct SimplePath {
    dbl_colon_opt: Option<PuncKind>,
    root: PathSegment,
    segments: Vec<(PuncKind, PathSegment)>,
}