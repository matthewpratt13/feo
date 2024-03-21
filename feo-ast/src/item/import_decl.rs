use feo_types::{
    span::{Span, Spanned},
    type_utils::{ColonColonAsterisk, KwImport, Semicolon},
};

use crate::{attribute::OuterAttr, path::SimplePath};

use super::{PathCollection, VisibilityKind};

#[derive(Debug, Clone)]
pub enum ImportTree {
    SimplePath(SimplePath),
    Subset(PathSubset),
    Wildcard(PathWildcard),
    Recursive(PathRecursive),
}

impl Spanned for ImportTree {
    fn span(&self) -> Span {
        match self {
            ImportTree::SimplePath(sp) => sp.span(),
            ImportTree::Subset(ps) => ps.span(),
            ImportTree::Wildcard(pw) => pw.span(),
            ImportTree::Recursive(pr) => pr.span(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_import: KwImport,
    pub import_tree: ImportTree,
    pub semicolon: Semicolon,
}

impl Spanned for ImportDecl {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_import.span(),
                },
            },
            None => self.kw_import.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathWildcard {
    pub path_prefix: PathCollection<SimplePath>,
    pub colon_colon_asterisk: ColonColonAsterisk,
}

impl Spanned for PathWildcard {
    fn span(&self) -> Span {
        let s1 = self.path_prefix.span();
        let s2 = self.colon_colon_asterisk.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathRecursive(pub PathCollection<ImportTree>);

impl Spanned for PathRecursive {
    fn span(&self) -> Span {
        self.0.span()
    }
}

#[derive(Debug, Clone)]
pub struct PathSubset(pub PathCollection<SimplePath>);

impl Spanned for PathSubset {
    fn span(&self) -> Span {
        self.0.span()
    }
}
