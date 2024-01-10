use crate::{
    expression::{ExprWithoutBlock, Expression},
    identifier::Identifier,
    item::Item,
    keyword::KeywordKind,
    punctuation::PuncKind,
    type_annotation::TypeAnnKind,
};

pub enum Statement {
    Item(Item),
    Expr(ExprStatement),
    Let(LetStatement),
}

pub struct ExprStatement {
    expr_without_block: ExprWithoutBlock,
    semicolon: PuncKind,
}

pub struct LetStatement {
    kw_let: KeywordKind,
    kw_mut_opt: Option<KeywordKind>,
    identifier: Identifier,
    type_ann_opt: Option<(PuncKind, TypeAnnKind)>,
    equals: PuncKind,
    expression: Expression,
    semicolon: PuncKind,
}

pub struct StatementWithExpr {
    statement: Statement,
    expr_without_block: ExprWithoutBlock
}
