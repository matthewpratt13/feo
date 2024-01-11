use crate::{identifier::Identifier, keyword::KeywordKind};

use super::associated_item::AssociatedItem;
use super::Brace;

pub struct TraitItem {
    kw_unsafe_opt: Option<KeywordKind>,
    kw_impl: KeywordKind,
    name: Identifier,
    open_brace: Brace,
    associated_items: Vec<AssociatedItem>,
    close_brace: Brace,
}
