use feo_types::{
    span::{Span, Spanned},
    type_utils::{KwLet, Semicolon},
};

use crate::{attribute::OuterAttr, expression::Expression, item::Item, pattern::Pattern, ty::Type};

/// Component of a block, which is a component of an outer expression / function.
#[derive(Debug, Clone)]
pub enum Statement {
    Item(Item),
    ExprStatement(ExprStatement),
    LetStatement(LetStatement),
}

#[derive(Debug, Clone)]
pub struct ExprStatement {
    pub expression: Expression,
    pub semicolon_opt: Option<Semicolon>,
}

impl Spanned for ExprStatement {
    fn span(&self) -> Span {
        let start_pos = self.expression.span().start();

        let end_pos = if let Some(s) = &self.semicolon_opt {
            s.span().end()
        } else {
            self.expression.span().end()
        };

        let source = self.expression.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub kw_let: KwLet,
    pub pattern: Box<Pattern>,
    pub type_ann_opt: Option<Type>,
    pub assignment_opt: Option<Expression>,
    pub semicolon: Semicolon,
}

impl Spanned for LetStatement {
    fn span(&self) -> Span {
        let s1 = match &self.attributes_opt {
            Some(a) => match a.first() {
                Some(oa) => oa.span(),
                None => self.kw_let.span(),
            },
            None => self.kw_let.span(),
        };

        let s2 = self.semicolon.span();

        Span::join(s1, s2)
    }
}
