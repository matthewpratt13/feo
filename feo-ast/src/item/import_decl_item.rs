use feo_types::span::{Span, Spanned};

use crate::{
    identifier::Identifier,
    keyword::Keyword,
    path::SimplePath,
    program::LibraryItem,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{ImportTree, Item, VisibilityKind};

pub struct ImportDeclItem<T> {
    visibility_opt: Option<VisibilityKind>,
    kw_import: Keyword,
    import_tree: Box<dyn ImportTree<T>>,
    semicolon: Semicolon,
}

impl<T> Item for ImportDeclItem<T> {}

impl<T, L> LibraryItem<L> for ImportDeclItem<T> where L: Item {}

impl<T> Spanned for ImportDeclItem<T> {
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
    kw_as: Keyword,
    new_name: Identifier,
}

impl Item for PathWithAsClauseItem {}

impl<I> ImportTree<I> for PathWithAsClauseItem where I: Item {}
