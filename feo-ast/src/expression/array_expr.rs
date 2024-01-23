use crate::{
    literal::Literal,
    span::{Span, Spanned},
    statement::Statement,
    type_utils::{Bracket, Comma, Semicolon},
};

use super::{Constant, ExprWithoutBlock, Expression};

pub struct ArrayExpr {
    open_bracket: Bracket,
    elements_opt: Option<ArrayElements>,
    close_bracket: Bracket,
}

impl Expression for ArrayExpr {}

impl<E> ExprWithoutBlock<E> for ArrayExpr {}

impl Statement for ArrayExpr {}

impl Constant for ArrayExpr {}

impl Spanned for ArrayExpr {
    fn span(&self) -> Span {
        let start_pos = self.open_bracket.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.open_bracket.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct ArrayElements {
    first_element: Box<dyn Expression>,
    subsequent_elements: Vec<(Comma, Box<dyn Expression>)>,
    trailing_comma_opt: Option<Comma>,
}

pub struct ArrayDef {
    element_type: Box<dyn Expression>,
    semicolon: Semicolon,
    num_elements: Literal<u64>,
}

pub struct IndexExpr {
    object: Box<dyn Expression>,
    open_bracket: Bracket,
    index: Literal<u64>,
    close_bracket: Bracket,
}

impl Expression for IndexExpr {}

impl<E> ExprWithoutBlock<E> for IndexExpr {}

impl Statement for IndexExpr {}

impl Constant for IndexExpr {}

impl Spanned for IndexExpr {
    fn span(&self) -> Span {
        let start_pos = self.object.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.object.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
