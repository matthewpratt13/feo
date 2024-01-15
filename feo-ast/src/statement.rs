#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    item::{Colon, Equals, Item, Semicolon},
    keyword::KeywordKind,
    type_annotation::TypeAnnKind,
};

pub enum Statement {
    Expr(ExprStatement),
    Item(Item),
    Let(LetStatement),
}

pub struct ExprStatement {
    expr_without_block: ExprWithoutBlock,
    semicolon: Semicolon,
}

pub struct LetStatement {
    kw_let: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    identifier: Identifier,
    type_ann_opt: Option<(Colon, TypeAnnKind)>,
    equals: Equals,
    value: Box<Expression>,
    semicolon: Semicolon,
}

pub struct StatementWithExpr {
    statement: Box<Statement>,
    expr_without_block: ExprWithoutBlock,
}
