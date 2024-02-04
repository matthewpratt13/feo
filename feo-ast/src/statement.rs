#![allow(dead_code)]

use feo_types::{
    span::{Span, Spanned},
    utils::{Colon, Equals, KwLet, Semicolon},
};

use crate::{
    expression::{
        Assignable, BooleanOperand, Castable, Constant, ExprWithoutBlock, Expression, IterableExpr,
        OuterAttr,
    },
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

pub struct ExprStatement<
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
> {
    expression: Expression<A, B, C, E, I, S, U>,
    semicolon_opt: Option<Semicolon>,
}

impl<A, B, C, E, I, S, U> Statement for ExprStatement<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
}

impl<A, B, C, E, I, S, U> Constant for ExprStatement<A, B, C, E, I, S, U>
where
    A: Assignable + 'static,
    B: BooleanOperand + Spanned,
    C: Castable + 'static,
    E: ExprWithoutBlock + 'static,
    I: IterableExpr,
    S: Statement + 'static,
    U: Spanned + 'static,
{
}

impl<A, B, C, E, I, S, U> Spanned for ExprStatement<A, B, C, E, I, S, U>
where
    A: Assignable,
    B: BooleanOperand + Spanned,
    C: Castable,
    E: ExprWithoutBlock,
    I: IterableExpr,
    S: Statement,
    U: Spanned,
{
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

pub struct LetStatement<A: Assignable> {
    attributes: Vec<OuterAttr>,
    kw_let: KwLet,
    pattern: Box<dyn Pattern>,
    type_ann_opt: Option<(Colon, Box<dyn Type>)>,
    assignment_opt: Option<(Equals, A)>,
    semicolon: Semicolon,
}

impl<A> Statement for LetStatement<A> where A: Assignable {}

impl<A> Pattern for LetStatement<A> where A: Assignable {}

impl<A> Constant for LetStatement<A> where A: Assignable + 'static {}

impl<A> Spanned for LetStatement<A>
where
    A: Assignable,
{
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
