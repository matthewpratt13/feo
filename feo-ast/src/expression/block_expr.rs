use crate::{
    delimiter::{DelimKind, DelimOrientation},
    statement::{Statement, StatementWithExpr},
};

use super::ExprWithoutBlock;

pub enum Statements {
    Statement(Box<Statement>),
    StatementWithExpr(Box<StatementWithExpr>),
    ExprWithoutBlock(ExprWithoutBlock),
}

pub struct BlockExpr {
    open_brace: (DelimKind, DelimOrientation),
    statements: Statements,
    close_brace: (DelimKind, DelimOrientation),
}
