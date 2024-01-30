use feo_types::{
    span::{Span, Spanned},
    utils::{Bracket, Comma, Semicolon},
};

use crate::{literal::Literal, statement::Statement, ty::ArrayType};

use super::{BooleanOperand, Constant, ExprWithoutBlock, Expression, IterableExpr};

pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

impl Expression for ArrayExpr {}

impl<E> ExprWithoutBlock<E> for ArrayExpr {}

impl Statement for ArrayExpr {}

impl BooleanOperand for ArrayExpr {}

impl Constant for ArrayExpr {}

impl IterableExpr for ArrayExpr {}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        let s1 = self.open_bracket.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)

        // let start_pos = self.open_bracket.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.open_bracket.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct ArrayElements {
    first_element: Box<dyn Expression>,
    subsequent_elements: Vec<(Comma, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayWithSingleRepeatedValue {
    repeated_value: Box<dyn Expression>,
    semicolon: Semicolon,
    num_of_repeats: Literal<u64>,
}

pub struct IndexExpr {
    operand: ArrayType,
    open_bracket: Bracket,
    index: Literal<u64>,
    close_bracket: Bracket,
}

impl Expression for IndexExpr {}

impl<E> ExprWithoutBlock<E> for IndexExpr {}

impl Statement for IndexExpr {}

impl BooleanOperand for IndexExpr {}

impl Constant for IndexExpr {}

impl IterableExpr for IndexExpr {}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let s1 = self.operand.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)

        // let start_pos = self.operand.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.operand.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
