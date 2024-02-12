use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, Parenthesis},
    Identifier,
};

use crate::path::PathInExpr;

use super::{Expression, OuterAttr};

#[derive(Clone)]
pub enum StructExprKind {
    Struct(StructExpr),
    TupleStruct(TupleStructExpr),
    UnitStruct(UnitStructExpr),
}

impl Spanned for StructExprKind {
    fn span(&self) -> Span {
        match self {
            StructExprKind::Struct(s) => s.span(),
            StructExprKind::TupleStruct(ts) => ts.span(),
            StructExprKind::UnitStruct(us) => us.span(),
        }
    }
}

#[derive(Clone)]
pub struct StructExpr {
    pub item_path: PathInExpr,
    pub open_brace: Brace,
    pub struct_expr_fields_opt: Option<StructExprFields>,
    pub close_brace: Brace,
}

impl Spanned for StructExpr {
    fn span(&self) -> Span {
        let s1 = self.item_path.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct StructExprField(pub Vec<OuterAttr>, pub (Identifier, Colon, Box<Expression>));

#[derive(Clone)]
pub struct StructExprFields {
    pub first_field: StructExprField,
    pub subsequent_fields: Vec<(Comma, StructExprField)>,
}

#[derive(Clone)]
pub struct TupleStructExpr {
    item_path: PathInExpr,
    open_parenthesis: Parenthesis,
    params_opt: Option<(Box<Expression>, Vec<(Comma, Expression)>, Option<Comma>)>,
    close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructExpr {
    fn span(&self) -> Span {
        let s1 = self.item_path.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}

#[derive(Clone)]
pub struct UnitStructExpr(PathInExpr);

impl Spanned for UnitStructExpr {
    fn span(&self) -> Span {
        let s1 = self.0.span();
        let s2 = self.0.span();

        Span::join(s1, s2)
    }
}
