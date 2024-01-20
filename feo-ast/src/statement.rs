#![allow(dead_code)]

use feo_types::span::{Span, Spanned};

use crate::{
    expression::{Constant, ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::Keyword,
    pattern::Pattern,
    type_annotation::TypeAnnKind,
    type_utils::{Colon, Equals, Semicolon},
};

pub trait Statement
where
    Self: Spanned,
{
}

pub struct ExprStatement<T> {
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
    semicolon: Semicolon,
}

impl<T: 'static> Constant for ExprStatement<T> {}

impl<T> Statement for ExprStatement<T> {}

impl<T> Spanned for ExprStatement<T> {
    fn span(&self) -> Span {
        let start_pos = self.expr_without_block.span().start();
        let end_pos = self.semicolon.span().end();
        let source = self.expr_without_block.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct LetStatement {
    kw_let: Keyword,
    kw_mut_opt: Option<Keyword>,
    identifier: Identifier,
    type_ann_opt: Option<(Colon, TypeAnnKind)>,
    equals: Equals,
    value: Box<dyn Expression>,
    semicolon: Semicolon,
}

impl Constant for LetStatement {}

impl Pattern for LetStatement {}

impl Statement for LetStatement {}

impl Spanned for LetStatement {
    fn span(&self) -> Span {
        let start_pos = self.kw_let.span().start();
        let end_pos = self.semicolon.span().end();
        let source = self.kw_let.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StatementWithExpr<T> {
    statement: Box<dyn Statement>,
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
}

impl<T> Statement for StatementWithExpr<T> {}

impl<T> Spanned for StatementWithExpr<T> {
    fn span(&self) -> Span {
        let start_pos = self.statement.span().start();
        let end_pos = self.expr_without_block.span().end();
        let source = self.statement.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
