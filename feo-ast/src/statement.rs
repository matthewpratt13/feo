#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    item::ItemKind,
    keyword::KeywordKind,
    type_annotation::TypeAnnKind,
    type_utils::{Colon, Equals, Semicolon},
};

pub enum StatementKind<T> {
    Expr(ExprStatement<T>),
    Item(ItemKind),
    Let(LetStatement),
}

pub struct ExprStatement<T> {
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
    semicolon: Semicolon,
}

pub struct LetStatement {
    kw_let: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    identifier: Identifier,
    type_ann_opt: Option<(Colon, TypeAnnKind)>,
    equals: Equals,
    value: Box<dyn Expression>,
    semicolon: Semicolon,
}

pub struct StatementWithExpr<T, U> {
    statement: Box<StatementKind<T>>,
    expr_without_block: Box<dyn ExprWithoutBlock<U>>,
}
