use crate::{
    statement::{StatementKind, StatementWithExpr},
    type_utils::Brace,
};

use super::ExprWithoutBlockKind;

pub enum StatementsKind {
    ExprWithoutBlock(ExprWithoutBlockKind),
    Statement(Box<StatementKind>),
    StatementWithExpr(StatementWithExpr),
}

pub struct BlockExpr {
    open_brace: Brace,
    statements: StatementsKind,
    close_brace: Brace,
}
