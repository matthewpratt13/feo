use crate::{
    item::Brace,
    statement::{Statement, StatementWithExpr},
};

use super::ExprWithoutBlock;

pub enum Statements {
    Statement(Box<Statement>),
    StatementWithExpr(StatementWithExpr),
    ExprWithoutBlock(ExprWithoutBlock),
}

pub struct BlockExpr {
    open_brace: Brace,
    statements: Statements,
    close_brace: Brace,
}
