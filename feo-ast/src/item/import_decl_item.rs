use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    punctuation::PuncKind,
};

pub enum ImportTree {
    EntirePath(EntirePath),
    PathSubsetRecursive(PathSubsetRecursive),
    PathWithAsClause(PathWithAsClause),
}

pub struct ImportDeclItem {
    kw_import: KeywordKind,
    import_tree: ImportTree,
    semicolon: PuncKind,
}

pub struct EntirePath {
    path: Vec<Option<(Option<SimplePath>, PuncKind)>>,
    asterisk: PuncKind,
}

pub struct PathSubsetRecursive {
    path_root_opt: Option<(Option<SimplePath>, PuncKind)>,
    open_brace: (DelimKind, DelimOrientation),
    recursive_tree_opt: Option<(
        Box<ImportTree>,
        Vec<(PuncKind, ImportTree)>,
        Option<PuncKind>,
    )>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct PathWithAsClause {
    path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}
