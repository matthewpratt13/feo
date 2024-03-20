use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, ColonColonAsterisk, KwImport, Semicolon},
};

use crate::{attribute::OuterAttr, expression::TermCollection, path::SimplePath};

use super::VisibilityKind;

#[derive(Debug, Clone)]
pub enum ImportTree {
    SimplePath(SimplePath),
    Wildcard(PathWildcard),
    SubsetRecursive(PathSubsetRecursive),
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
    pub full_path: SimplePath,
    pub colon_colon_asterisk: ColonColonAsterisk,
}

impl Spanned for PathWildcard {
    fn span(&self) -> Span {
        let s1 = self.full_path.span();
        let s2 = self.colon_colon_asterisk.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathSubsetRecursive {
    pub path_prefix: SimplePath,
    pub open_brace: Brace,
    pub recursive_tree_opt: Option<Box<TermCollection<ImportTree>>>,
    pub close_brace: Brace,
}

impl Spanned for PathSubsetRecursive {
    fn span(&self) -> Span {
        let s1 = self.path_prefix.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct PathSubset {
    pub path_prefix: SimplePath,
    pub open_brace: Brace,
    pub inner_paths: TermCollection<SimplePath>,
    pub close_brace: Brace,
}

impl Spanned for PathSubset{
    fn span(&self) -> Span {
        let s1 = self.path_prefix.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

