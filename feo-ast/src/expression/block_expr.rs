use crate::{
    statement::{Statement, StatementWithExpr},
    type_utils::Brace,
};

use super::{ExprWithBlock, ExprWithoutBlock, Expression};

pub enum StatementsKind<T, U> {
    ExprWithoutBlock(Box<dyn ExprWithoutBlock<T>>),
    Statement(Box<dyn Statement>),
    StatementWithExpr(StatementWithExpr<U>),
}

pub struct BlockExpr<T, U> {
    open_brace: Brace,
    statements: StatementsKind<T, U>,
    close_brace: Brace,
}

impl<T, U> Expression for BlockExpr<T, U> {}

impl<T, U, E> ExprWithBlock<E> for BlockExpr<T, U> where E: Expression {}
