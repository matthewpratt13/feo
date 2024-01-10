use crate::{punctuation::PuncKind, statement::{Statement, StatementWithExpr}};

use super::ExprWithoutBlock;

pub struct BlockExpr {
    open_brace: PuncKind,
    statements: Statements,
    close_brace: PuncKind,
}

pub enum Statements {
    Statement(Box<Statement>),
    StatementWithExpr(Box<StatementWithExpr>),
    ExprWithoutBlock(ExprWithoutBlock),
}
