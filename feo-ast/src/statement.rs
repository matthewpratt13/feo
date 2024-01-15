#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::KeywordKind,
    type_annotation::TypeAnnKind,
    type_utils::{Colon, Equals, Semicolon},
};

pub trait Statement {}

impl<T> Statement for ExprStatement<T> {}

impl Statement for LetStatement {}

impl<T> Statement for StatementWithExpr<T> {}

// pub enum StatementKind<T> {
//     Expr(ExprStatement<T>),
//     Item(Box<dyn Item>),
//     Let(LetStatement),
// }

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

pub struct StatementWithExpr<T> {
    statement: Box<dyn Statement>,
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
}
