use feo_types::span::{Position, Span, Spanned};

use crate::{
    keyword::Keyword,
    path::SimplePath,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{AsClause, ImportTree, Item, VisibilityKind};

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

pub struct PathWildcard {
    path_prefix: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

impl ContractItem for PathWildcard {}

impl ImportTree for PathWildcard {}

impl Item for PathWildcard {}

impl Statement for PathWildcard {}

impl Spanned for PathWildcard {
    fn span(&self) -> Span {
        let start_pos = match self.path_prefix.first() {
            Some(p) => match p {
                Some(q) => match &q.0 {
                    Some(r) => r.span().start(),
                    None => self.asterisk.span().start(),
                },
                None => self.asterisk.span().start(),
            },
            None => self.asterisk.span().start(),
        };

        let end_pos = self.asterisk.span().end();
        let source = self.asterisk.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct PathSubsetRecursive {
    path_prefix_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(
        Box<dyn ImportTree>,
        Vec<(Comma, Box<dyn ImportTree>)>,
        Option<Comma>,
    )>,
    close_brace: Brace,
}

impl ContractItem for PathSubsetRecursive {}

impl ImportTree for PathSubsetRecursive {}

impl Item for PathSubsetRecursive {}

impl Statement for PathSubsetRecursive {}

impl Spanned for PathSubsetRecursive {
    fn span(&self) -> Span {
        let start_pos = match &self.path_prefix_opt {
            Some(p) => match &p.0 {
                Some(q) => q.span().start(),
                None => self.open_brace.span().start(),
            },
            None => self.open_brace.span().start(),
        };

        let end_pos = self.close_brace.span().end();
        let source = self.open_brace.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct PathWithAsClause {
    path_prefix: SimplePath,
    as_clause_opt: Option<AsClause>,
}

impl ContractItem for PathWithAsClause {}

impl ImportTree for PathWithAsClause {}

impl Item for PathWithAsClause {}

impl Statement for PathWithAsClause {}

impl Spanned for PathWithAsClause {
    fn span(&self) -> Span {
        let start_pos = self.path_prefix.span().start();
        let end_pos = if let Some(a) = self.as_clause_opt {
            a.span().end()
        } else {
            self.path_prefix.span().end()
        };

        let source = self.path_prefix.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
