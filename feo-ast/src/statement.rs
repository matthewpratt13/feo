#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlockKind, ExpressionKind},
    identifier::Identifier,
    item::ItemKind,
    keyword::KeywordKind,
    type_annotation::TypeAnnKind,
    type_utils::{Colon, Equals, Semicolon},
};

pub enum StatementKind {
    Expr(ExprStatement),
    Item(ItemKind),
    Let(LetStatement),
}

pub struct ExprStatement {
    expr_without_block: ExprWithoutBlockKind,
    semicolon: Semicolon,
}

pub struct LetStatement {
    kw_let: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    identifier: Identifier,
    type_ann_opt: Option<(Colon, TypeAnnKind)>,
    equals: Equals,
    value: Box<ExpressionKind>,
    semicolon: Semicolon,
}

pub struct StatementWithExpr {
    statement: Box<StatementKind>,
    expr_without_block: ExprWithoutBlockKind,
}
