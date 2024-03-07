use feo_types::{
    span::{Span, Spanned},
    utils::{Asterisk, Brace, Comma, DblColon, KwImport, Semicolon},
};

use crate::path::SimplePath;

use super::{AsClause, VisibilityKind};

#[derive(Debug, Clone)]
pub enum ImportTree {
    Wildcard(PathWildcard),
    SubsetRecursive(PathSubsetRecursive),
    WithAsClause(PathWithAsClause),
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    visibility_opt: Option<VisibilityKind>,
    kw_import: KwImport,
    import_tree: ImportTree,
    semicolon: Semicolon,
}

impl Spanned for ImportDecl {
    fn span(&self) -> Span {
        let s1 = if let Some(v) = &self.visibility_opt {
            v.span()
        } else {
            self.kw_import.span()
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathWildcard {
    full_path: Vec<Option<(Option<SimplePath>, DblColon)>>,
    asterisk: Asterisk,
}

impl Spanned for PathWildcard {
    fn span(&self) -> Span {
        let s1 = match self.full_path.first() {
            Some(p) => match p {
                Some(q) => match &q.0 {
                    Some(r) => r.span(),
                    None => self.asterisk.span(),
                },
                None => self.asterisk.span(),
            },
            None => self.asterisk.span(),
        };

        let s2 = self.asterisk.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathSubsetRecursive {
    path_prefix_opt: Option<(Option<SimplePath>, DblColon)>,
    open_brace: Brace,
    recursive_tree_opt: Option<(Box<ImportTree>, Vec<(Comma, ImportTree)>, Option<Comma>)>,
    close_brace: Brace,
}

impl Spanned for PathSubsetRecursive {
    fn span(&self) -> Span {
        let s1 = match &self.path_prefix_opt {
            Some(p) => match &p.0 {
                Some(q) => q.span(),
                None => self.open_brace.span(),
            },
            None => self.open_brace.span(),
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathWithAsClause {
    path_prefix: SimplePath,
    as_clause_opt: Option<AsClause>,
}

impl Spanned for PathWithAsClause {
    fn span(&self) -> Span {
        let s1 = self.path_prefix.span();
        let s2 = if let Some(a) = &self.as_clause_opt {
            a.span()
        } else {
            self.path_prefix.span()
        };

        Span::join(s1, s2)
    }
}
