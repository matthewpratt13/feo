#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, Semicolon},
    Keyword,
};

use crate::{
    expression::{Assignable, Constant, Expression, OuterAttr},
    pattern::Pattern,
    ty::Type,
};

// statement: component of a block, which is a component of an outer expression / function

// statements:
//  - let declaration
//  - item declaration
//  - expression statement

pub trait Statement
where
    Self: Spanned,
{
}

pub struct ExprStatement {
    expression: Box<dyn Expression>,
    semicolon_opt: Option<Semicolon>,
}

impl Statement for ExprStatement {}

impl Constant for ExprStatement {}

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

pub struct LetStatement {
    attributes: Vec<OuterAttr>,
    kw_let: Keyword,
    pattern: Box<dyn Pattern>,
    type_ann_opt: Option<(Colon, Box<dyn Type>)>,
    assignment_opt: Option<(Equals, Box<dyn Assignable>)>,
    semicolon: Semicolon,
}

impl Statement for LetStatement {}

impl Pattern for LetStatement {}

impl Constant for LetStatement {}

impl Spanned for LetStatement {
    fn span(&self) -> Span {
        let start_pos = if let Some(a) = self.attributes.first() {
            a.span().start()
        } else {
            self.kw_let.span().start()
        };

        let end_pos = self.semicolon.span().end();
        let source = self.kw_let.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
