use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, KwMod, Semicolon},
    Identifier,
};

use crate::attribute::OuterAttr;

use super::{Item, VisibilityKind};

#[derive(Debug, Clone)]
pub struct ModWithBody {
    attributes_opt: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_mod: KwMod,
    mod_name: Identifier,
    open_brace: Brace,
    items: Box<Item>,
    close_brace: Brace,
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

#[derive(Debug, Clone)]
pub struct ModWithoutBody {
    attributes_opt: Option<Vec<OuterAttr>>,
    visibility_opt: Option<VisibilityKind>,
    kw_mod: KwMod,
    mod_name: Identifier,
    semicolon: Semicolon,
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
