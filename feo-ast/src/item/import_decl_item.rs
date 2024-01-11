use crate::{
    delimiter::{DelimKind, DelimOrientation},
    identifier::Identifier,
    keyword::KeywordKind,
    path::SimplePath,
    punctuation::PuncKind,
};

pub enum ImportTree {
    EntirePath(ImportEntirePath),
    PathSubset(ImportPathSubset),
    PathWithAsClause(ImportPathWithAsClause),
}

pub struct ImportDeclItem {
    kw_import: KeywordKind,
    import_tree: ImportTree,
    semicolon: PuncKind,
}

pub struct ImportEntirePath {
    path: Vec<Option<(Option<SimplePath>, PuncKind)>>,
    asterisk: PuncKind,
}

pub struct ImportPathSubset {
    path_root_opt: Option<(Option<SimplePath>, PuncKind)>,
    open_brace: (DelimKind, DelimOrientation),
    tree: Option<(
        Box<ImportTree>,
        Vec<(PuncKind, ImportTree)>,
        Option<PuncKind>,
    )>,
    close_brace: (DelimKind, DelimOrientation),
}

pub struct ImportPathWithAsClause {
    path_root: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: KeywordKind,
    new_name: Identifier,
}
