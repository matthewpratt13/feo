use feo_types::span::{Span, Spanned};

use crate::{
    keyword::Keyword,
    path::SimplePath,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{ImportTree, Item, VisibilityKind};

pub struct ImportDecl {
    visibility_opt: Option<VisibilityKind>,
    kw_import: Keyword,
    import_tree: Box<dyn ImportTree>,
    semicolon: Semicolon,
}

impl ContractItem for ImportDecl {}

impl Item for ImportDecl {}

impl LibraryItem for ImportDecl {}

impl Statement for ImportDecl {}

impl Spanned for ImportDecl {
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
    item_path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

pub struct PathSubsetRecursiveItem {
    item_path_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Vec<(Comma, Box<dyn ImportTree>)>, Option<Comma>)>,
    close_brace: Brace,
}
