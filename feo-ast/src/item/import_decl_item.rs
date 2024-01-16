use crate::{
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{ImportTree, VisibilityKind};

pub struct ImportDeclItem<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_import: KeywordKind,
    import_tree: Box<dyn ImportTree<T>>,
    semicolon: Semicolon,
}

pub struct EntirePathContentsItem {
    path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

pub struct PathSubsetRecursiveItem<T> {
    path_root_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(
        Box<dyn ImportTree<T>>,
        Vec<(Comma, Box<dyn ImportTree<T>>)>,
        Option<Comma>,
    )>,
    close_brace: Brace,
}

pub struct PathWithAsClauseItem {
    original_path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}
