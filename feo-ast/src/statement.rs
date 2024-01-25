#![allow(dead_code)]

use crate::{
    expression::{Assignable, Constant, ExprWithoutBlock, OuterAttr},
    keyword::Keyword,
    pattern::Pattern,
    span::{Span, Spanned},
    ty::Type,
    type_utils::{Colon, Equals, Semicolon},
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

pub struct ExprStatement<T> {
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
    semicolon_opt: Option<Semicolon>,
}

impl<T> Statement for ExprStatement<T> {}

impl<T> Constant for ExprStatement<T> where T: 'static {}

impl<T> Spanned for ExprStatement<T> {
    fn span(&self) -> Span {
        let start_pos = self.expr_without_block.span().start();

        let end_pos = if let Some(s) = &self.semicolon_opt {
            s.span().end()
        } else {
            self.expr_without_block.span().end()
        };

        let source = self.expr_without_block.span().source();

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

pub struct StatementsWithExpr<T> {
    statements: Vec<Box<dyn Statement>>,
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
}

impl<T> Statement for StatementsWithExpr<T> {}

impl<T> Spanned for StatementsWithExpr<T> {
    fn span(&self) -> Span {
        let start_pos = if let Some(s) = self.statements.first() {
            s.span().start()
        } else {
            self.expr_without_block.span().start()
        };

        let end_pos = self.expr_without_block.span().end();
        let source = self.expr_without_block.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
