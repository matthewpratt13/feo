use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwMod, Semicolon},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::{Item, VisibilityKind};

#[derive(Debug, Clone)]
pub struct ModWithBody {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_mod: KwMod,
    pub mod_name: Identifier,
    pub open_brace: Brace,
    pub items_opt: Option<Vec<Item>>,
    pub close_brace: Brace,
}

#[derive(Debug, Clone)]
pub struct ModWithoutBody {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub visibility_opt: Option<VisibilityKind>,
    pub kw_mod: KwMod,
    pub mod_name: Identifier,
    pub semicolon: Semicolon,
}

impl Spanned for ModWithoutBody {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_mod.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_mod.span(),
            },
        };
        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}

impl Spanned for ModWithBody {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => match &self.visibility_opt {
                    Some(v) => v.span(),
                    None => self.kw_mod.span(),
                },
            },
            None => match &self.visibility_opt {
                Some(v) => v.span(),
                None => self.kw_mod.span(),
            },
        };

        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}
