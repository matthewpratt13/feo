use feo_types::span::{Span, Spanned};

use crate::{
    expression::{InnerAttr, OuterAttr},
    identifier::Identifier,
    keyword::Keyword,
    program::{ContractItem, LibraryItem},
    statement::Statement,
    type_utils::{Brace, Semicolon},
};

use super::{Item, ModItem, VisibilityKind};

pub struct ModWithBody {
    outer_attributes: Vec<OuterAttr>,
    visibility_opt: Option<VisibilityKind>,
    kw_mod: Keyword,
    identifier: Identifier,
    open_brace: Brace,
    inner_attributes: Vec<InnerAttr>,
    items: Vec<Box<dyn Item>>,
    close_brace: Brace,
}

impl ContractItem for ModWithBody {}

impl Item for ModWithBody {}

impl LibraryItem for ModWithBody {}

impl ModItem for ModWithBody {}

impl Statement for ModWithBody {}

impl Spanned for ModWithBody {
    fn span(&self) -> Span {
        let start_pos = match self.outer_attributes.first() {
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
    kw_mod: Keyword,
    identifier: Identifier,
    semicolon: Semicolon,
}

impl ContractItem for ModWithoutBody {}

impl Item for ModWithoutBody {}

impl LibraryItem for ModWithoutBody {}

impl ModItem for ModWithoutBody {}

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
