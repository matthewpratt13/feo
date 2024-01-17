use feo_types::span::{Span, Spanned};

use crate::{
    identifier::Identifier,
    path::SimplePath,
    type_utils::{Brace, Colon, Comma, Parenthesis},
};

use super::{ExprWithoutBlock, Expression, OuterAttr, StructExpr};

pub struct Struct {
    struct_path: SimplePath,
    open_brace: Brace,
    struct_expr_fields_opt: Option<StructExprFields>,
    close_brace: Brace,
}

impl Expression for Struct {}

impl<E> ExprWithoutBlock<E> for Struct {}

impl<S, E> StructExpr<S, E> for Struct {}

impl Spanned for Struct {
    fn span(&self) -> Span {
        let start_pos = self.struct_path.span().start();
        let end_pos = self.close_brace.span().end();
        let source = self.struct_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct StructExprFields {
    first_field: StructExprField,
    subsequent_fields: Vec<(Comma, StructExprField)>,
}

pub struct StructExprField {
    attributes: Vec<OuterAttr>,
    data: (Identifier, Colon, Box<dyn Expression>),
}

pub struct TupleStruct {
    tuple_struct_path: SimplePath,
    open_parenthesis: Parenthesis,
    params_opt: Option<(
        Box<dyn Expression>,
        Vec<(Comma, Box<dyn Expression>)>,
        Option<Comma>,
    )>,
    close_parenthesis: Parenthesis,
}

impl Expression for TupleStruct {}

impl<E> ExprWithoutBlock<E> for TupleStruct {}

impl<S, E> StructExpr<S, E> for TupleStruct {}

impl Spanned for TupleStruct {
    fn span(&self) -> Span {
        let start_pos = self.tuple_struct_path.span().start();
        let end_pos = self.close_parenthesis.span().end();
        let source = self.tuple_struct_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct UnitStruct(SimplePath);

impl Expression for UnitStruct {}

impl<E> ExprWithoutBlock<E> for UnitStruct {}

impl<S, E> StructExpr<S, E> for UnitStruct {}

impl Spanned for UnitStruct {
    fn span(&self) -> Span {
        let start_pos = self.0.span().start();
        let end_pos = self.0.span().end();
        let source = self.0.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
