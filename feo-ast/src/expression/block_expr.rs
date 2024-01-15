use crate::{
    statement::{StatementKind, StatementWithExpr},
    type_utils::Brace,
};

use super::ExprWithoutBlock;


pub enum StatementsKind<T> {
    ExprWithoutBlock(Box<dyn ExprWithoutBlock<T>>),
    Statement(Box<StatementKind>),
    StatementWithExpr(StatementWithExpr),
}

pub struct BlockExpr<T> {
    open_brace: Brace,
    statements: StatementsKind<T>,
    close_brace: Brace,
}
