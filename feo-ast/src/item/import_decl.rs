use crate::{
    keyword::Keyword,
    path::SimplePath,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{Asterisk, Brace, Comma, DblColon, Semicolon},
};

use super::{AsClause, Item, VisibilityKind};

pub enum ImportTreeKind {
    Wildcard(PathWildcard),
    SubsetRecursive(PathSubsetRecursive),
    WithAsClause(PathWithAsClause),
}

pub struct ImportDecl {
    visibility_opt: Option<VisibilityKind>,
    kw_import: Keyword,
    import_tree: ImportTreeKind,
    semicolon: Semicolon,
}

impl Item for ImportDecl {}

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
    full_path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

impl Item for PathWildcard {}

impl Statement for PathWildcard {}

impl Spanned for PathWildcard {
    fn span(&self) -> Span {
        let start_pos = match self.full_path.first() {
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
    recursive_tree_opt: Option<(Box<ImportTreeKind>, Vec<(Comma, ImportTreeKind)>, Option<Comma>)>,
    close_brace: Brace,
}

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

impl Item for PathWithAsClause {}

impl Statement for PathWithAsClause {}

impl Spanned for PathWithAsClause {
    fn span(&self) -> Span {
        let start_pos = self.path_prefix.span().start();
        let end_pos = if let Some(a) = &self.as_clause_opt {
            a.span().end()
        } else {
            self.path_prefix.span().end()
        };

        let source = self.path_prefix.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
