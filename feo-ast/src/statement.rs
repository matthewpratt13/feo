#![allow(dead_code)]

use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    keyword::KeywordKind,
    type_annotation::TypeAnnKind,
    type_utils::{Colon, Equals, Semicolon},
};

pub trait Statement {}

pub struct ExprStatement<T> {
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
    semicolon: Semicolon,
}

impl<T> Statement for ExprStatement<T> {}

pub struct LetStatement {
    kw_let: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    identifier: Identifier,
    type_ann_opt: Option<(Colon, TypeAnnKind)>,
    equals: Equals,
    value: Box<dyn Expression>,
    semicolon: Semicolon,
}

impl Statement for LetStatement {}

pub struct StatementWithExpr<T> {
    statement: Box<dyn Statement>,
    expr_without_block: Box<dyn ExprWithoutBlock<T>>,
}

impl<T> Statement for StatementWithExpr<T> {}
