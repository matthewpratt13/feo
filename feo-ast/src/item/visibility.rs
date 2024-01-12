use crate::keyword::KeywordKind;

use super::Parenthesis;

pub enum Visibility {
    Pub(KeywordKind),
    PubCrate(PubCrateVisibility),
}

pub struct PubCrateVisibility {
    kw_pub: KeywordKind,
    open_parenthesis: Parenthesis,
    kw_crate: KeywordKind,
    close_parenthesis: Parenthesis,
}
