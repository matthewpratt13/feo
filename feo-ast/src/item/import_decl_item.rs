use feo_types::span::{Span, Spanned};

use crate::{
    identifier::Identifier,
    keyword::Keyword,
    path::SimplePath,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{ImportTree, Item, VisibilityKind};

pub struct ImportDeclItem {
    visibility_opt: Option<VisibilityKind>,
    kw_import: Keyword,
    import_tree: Box<dyn ImportTree>,
    semicolon: Semicolon,
}

impl ContractItem for ImportDeclItem {}

impl Item for ImportDeclItem {}

impl LibraryItem for ImportDeclItem {}

impl Statement for ImportDeclItem {}

impl Spanned for ImportDeclItem {
    fn span(&self) -> Span {
        let start_pos = if let Some(v) = &self.visibility_opt {
            v.span().start()
        } else {
            self.kw_import.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_import.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct EntirePathContentsItem {
    path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

pub struct PathSubsetRecursiveItem {
    path_root_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Vec<(Comma, Box<dyn ImportTree>)>, Option<Comma>)>,
    close_brace: Brace,
}

pub struct PathWithAsClauseItem {
    original_path: SimplePath,
    as_clause_opt: Option<AsClause>,
}

pub struct AsClause {
    kw_as: Keyword,
    new_name: Identifier,
}
