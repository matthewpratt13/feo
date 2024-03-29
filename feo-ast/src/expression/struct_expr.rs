use feo_types::{
    span::{Span, Spanned},
    type_utils::{Brace, Parenthesis},
    Identifier,
};

use crate::{attribute::OuterAttr, path::PathInExpr};

use super::{TermCollection, Value, ValueCollection};

#[derive(Debug, Clone)]
pub struct StructExpr {
    pub path: PathInExpr,
    pub open_brace: Brace,
    pub fields_opt: Option<TermCollection<StructExprField>>,
    pub close_brace: Brace,
}

impl Spanned for StructExpr {
    fn span(&self) -> Span {
        let s1 = self.path.span();
        let s2 = self.close_brace.span();

        Span::join(s1, s2)
    }
}

#[derive(Debug, Clone)]
pub struct StructExprField {
    pub attributes_opt: Option<Vec<OuterAttr>>,
    pub field_content: (Identifier, Box<Value>),
}

#[derive(Debug, Clone)]
pub struct TupleStructExpr {
    pub path: PathInExpr,
    pub open_parenthesis: Parenthesis,
    pub fields_opt: Option<ValueCollection>,
    pub close_parenthesis: Parenthesis,
}

impl Spanned for TupleStructExpr {
    fn span(&self) -> Span {
        let s1 = self.path.span();
        let s2 = self.close_parenthesis.span();

        Span::join(s1, s2)
    }
}
