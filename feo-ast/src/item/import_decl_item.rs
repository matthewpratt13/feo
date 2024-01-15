use crate::{
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

pub enum ImportTreeKind {
    EntirePathContent(EntirePathContents),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
}

pub struct ImportDeclItem {
    kw_import: KeywordKind,
    import_tree: ImportTreeKind,
    semicolon: Semicolon,
}

pub struct EntirePathContents {
    path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

pub struct PathSubsetRecursive {
    path_root_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Box<ImportTreeKind>, Vec<(Comma, ImportTreeKind)>, Option<Comma>)>,
    close_brace: Brace,
}

pub struct PathWithAsClause {
    original_path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}
