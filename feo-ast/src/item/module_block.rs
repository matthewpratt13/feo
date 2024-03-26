use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwModule, Semicolon},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::{Item, VisibilityKind};

#[derive(Debug, Clone)]
pub struct ModuleWithBlock {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_module: KwModule,
    pub module_name: Identifier,
    pub open_brace: Brace,
    pub items_opt: Option<Vec<Item>>,
    pub close_brace: Brace,
}

#[derive(Debug, Clone)]
pub struct ModuleWithoutBlock {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_module: KwModule,
    pub module_name: Identifier,
    pub semicolon: Semicolon,
}

impl Spanned for ModuleWithoutBlock {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_module.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_module.span(),
            },
        };
        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

impl Spanned for ModuleWithBlock {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_module.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_module.span(),
            },
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}
