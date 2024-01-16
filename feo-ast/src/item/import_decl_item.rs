use crate::{
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{ImportTree, Item, VisibilityKind};

pub struct ImportDeclItem<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_import: KeywordKind,
    import_tree: Box<dyn ImportTree<T>>,
    semicolon: Semicolon,
}

impl<T> Item for ImportDeclItem<T> {}

pub struct EntirePathContentsItem {
    path_opt: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

impl Item for EntirePathContentsItem {}

impl<I> ImportTree<I> for EntirePathContentsItem where I: Item {}

pub struct PathSubsetRecursiveItem<T> {
    path_root_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Vec<(Comma, Box<dyn ImportTree<T>>)>, Option<Comma>)>,
    close_brace: Brace,
}

impl<T> Item for PathSubsetRecursiveItem<T> {}

impl<T, I> ImportTree<I> for PathSubsetRecursiveItem<T> where I: Item {}

pub struct PathWithAsClauseItem {
    original_path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}

impl Item for PathWithAsClauseItem {}

impl<I> ImportTree<I> for PathWithAsClauseItem where I: Item {}
