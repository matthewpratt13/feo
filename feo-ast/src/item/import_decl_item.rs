use crate::{identifier::Identifier, keyword::KeywordKind, path::SimplePath};

use super::{Asterisk, Brace, Comma, DblColon, Semicolon};

pub enum ImportTree {
    EntirePath(EntirePath),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
}

pub struct ImportDeclItem {
    kw_import: KeywordKind,
    import_tree: ImportTree,
    semicolon: Semicolon,
}

pub struct EntirePath {
    path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

pub struct PathSubsetRecursive {
    path_root_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Box<ImportTree>, Vec<(Comma, ImportTree)>, Option<Comma>)>,
    close_brace: Brace,
}

pub struct PathWithAsClause {
    path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}
