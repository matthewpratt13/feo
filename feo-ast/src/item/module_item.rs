use feo_types::span::{Span, Spanned};

use crate::{
    expression::InnerAttr,
    identifier::Identifier,
    keyword::Keyword,
    program::LibraryItem,
    statement::Statement,
    type_utils::{Brace, Semicolon},
};

use super::{Item, ModItem, VisibilityKind};

pub struct ModWithBody {
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<Keyword>,
    kw_mod: Keyword,
    name: Identifier,
    open_brace: Brace,
    attributes: Vec<InnerAttr>,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

impl Item for ModWithBody {}

impl LibraryItem for ModWithBody {}

impl ModItem for ModWithBody {}

impl Statement for ModWithBody {}

impl Spanned for ModWithBody {
    fn span(&self) -> Span {
        let start_pos = match &self.visibility_opt {
            Some(v) => v.span().start(),
            None => match &self.kw_unsafe_opt {
                Some(ku) => ku.span().start(),
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
    visibility_opt: Option<VisibilityKind>,
    kw_unsafe_opt: Option<Keyword>,
    kw_mod: Keyword,
    name: Identifier,
    semicolon: Semicolon,
}

impl Item for ModWithoutBody {}

impl LibraryItem for ModWithoutBody {}

impl ModItem for ModWithoutBody {}

impl Statement for ModWithoutBody {}

impl Spanned for ModWithoutBody {
    fn span(&self) -> Span {
        let start_pos = match &self.visibility_opt {
            Some(v) => v.span().start(),
            None => match &self.kw_unsafe_opt {
                Some(ku) => ku.span().start(),
                None => self.kw_mod.span().start(),
            },
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_mod.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
