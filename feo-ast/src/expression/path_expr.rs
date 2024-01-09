use crate::{identifier::Identifier, keyword::KeywordKind, punctuation::PuncKind};

pub enum PathSegment {
    CrateSeg(KeywordKind),
    IdentifierSeg(Identifier),
    SelfSeg(KeywordKind),
    SuperSeg(KeywordKind),
}

pub struct SimplePath {
    dbl_colon_opt: Option<PuncKind>,
    root: PathSegment,
    segments: Vec<(PuncKind, PathSegment)>,
}

pub enum PathExpr {
    PathLoc(PathLocation),
    PathItem,
}

// TODO: Add `PathItem`

pub type PathLocation = SimplePath;
