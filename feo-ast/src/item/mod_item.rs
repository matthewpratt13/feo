use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, KwMod, Semicolon},
    Identifier,
};

use crate::{expression::OuterAttr, statement::Statement};

use super::{Item, VisibilityKind};

pub trait ModItem
where
    Self: Item + Sized,
{
}

pub struct ModWithBody {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_mod: KwMod,
    mod_name: Identifier,
    open_brace: Brace,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

impl ModItem for ModWithBody {}

impl Item for ModWithBody {}

impl Statement for ModWithBody {}

impl Spanned for ModWithBody {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_mod.span().start(),
            },
        };

        let end_pos = self.close_brace.span().end();
        let source = self.kw_mod.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ModWithoutBody {
    attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_mod: KwMod,
    mod_name: Identifier,
    semicolon: Semicolon,
}

impl ModItem for ModWithoutBody {}

impl Item for ModWithoutBody {}

impl Statement for ModWithoutBody {}

impl Spanned for ModWithoutBody {
    fn span(&self) -> Span {
        let start_pos = match self.attributes.first() {
            Some(a) => a.span().start(),
            None => match &self.visibility_opt {
                Some(v) => v.span().start(),
                None => self.kw_mod.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_mod.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
