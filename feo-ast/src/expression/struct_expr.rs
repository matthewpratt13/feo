use feo_types::{
    span::{Span, Spanned},
    utils::{Brace, Colon, Comma, Parenthesis},
    Identifier,
};

use crate::{path::PathInExpr, ty::Type};

use super::{Expression, OuterAttr};

pub enum StructKind {
    Struct(Struct),
    TupleStruct(TupleStruct),
    UnitStruct(UnitStruct),
}

impl Spanned for StructKind {
    fn span(&self) -> Span {
        match self {
            StructKind::Struct(s) => s.span(),
            StructKind::TupleStruct(ts) => ts.span(),
            StructKind::UnitStruct(us) => us.span(),
        }
    }
}

pub struct Struct {
    pub item_path: PathInExpr,
    pub open_brace: Brace,
    pub struct_expr_fields_opt: Option<StructExprFields>,
    pub close_brace: Brace,
}

impl Type for Struct {}

impl Spanned for Struct {
    fn span(&self) -> Span {
        let start_pos = self.item_path.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.item_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StructExprField(pub Vec<OuterAttr>, pub (Identifier, Colon, Box<Expression>));

pub struct StructExprFields {
    pub first_field: StructExprField,
    pub subsequent_fields: Vec<(Comma, StructExprField)>,
}

// pub struct StructExprField {
//     attributes: Vec<OuterAttr>,
//     data: (Identifier, Colon, Box<dyn Expression>),
// }

pub struct TupleStruct {
    item_path: PathInExpr,
    open_parenthesis: Parenthesis,
    params_opt: Option<(Box<Expression>, Vec<(Comma, Expression)>, Option<Comma>)>,
    close_parenthesis: Parenthesis,
}

impl Type for TupleStruct {}

impl Spanned for TupleStruct {
    fn span(&self) -> Span {
        let start_pos = self.item_path.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.item_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnitStruct(PathInExpr);

impl Type for UnitStruct {}

impl Spanned for UnitStruct {
    fn span(&self) -> Span {
        let start_pos = self.0.span().start();
        let end_pos = self.0.span().end();
        let source = self.0.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
