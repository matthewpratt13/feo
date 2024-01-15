use crate::{
    item::Brace,
    statement::{Statement, StatementWithExpr},
};

use super::ExprWithoutBlock;

pub enum Statements {
    ExprWithoutBlock(ExprWithoutBlock),
    Statement(Box<Statement>),
    StatementWithExpr(StatementWithExpr),
}

pub struct BlockExpr {
    open_brace: Brace,
    statements: Statements,
    close_brace: Brace,
}
