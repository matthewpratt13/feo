use feo_types::span::{Span, Spanned};

use crate::{
    path::SimplePath,
    statement::Statement,
    type_utils::{Bracket, HashBang, HashSign},
};

use super::{ExprWithBlock, ExprWithoutBlock, Expression};

pub struct InnerAttr {
    hash_bang: HashBang,
    open_bracket: Bracket,
    attribute_path: SimplePath,
    close_bracket: Bracket,
}

impl Expression for InnerAttr {}

impl<E> ExprWithoutBlock<E> for InnerAttr {}

impl Statement for InnerAttr {}

impl Spanned for InnerAttr {
    fn span(&self) -> Span {
        let start_pos = self.hash_bang.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.attribute_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}

pub struct OuterAttr {
    hash: HashSign,
    open_bracket: Bracket,
    attribute_path: SimplePath,
    close_bracket: Bracket,
}

impl Expression for OuterAttr {}

impl<E> ExprWithBlock<E> for OuterAttr {}

impl<E> ExprWithoutBlock<E> for OuterAttr {}

impl Statement for OuterAttr {}

impl Spanned for OuterAttr {
    fn span(&self) -> Span {
        let start_pos = self.hash.span().start();
        let end_pos = self.close_bracket.span().end();
        let source = self.attribute_path.span().source();

        let span = Span::new(source.as_str(), start_pos, end_pos);

        span
    }
}
