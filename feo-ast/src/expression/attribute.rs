use feo_types::{
    span::{Span, Spanned},
    utils::{Bracket, HashBang, HashSign, KwAbstract, KwExport, KwExtern, KwUnsafe},
};

use crate::path::SimplePath;

use super::{ExprWithBlock, ExprWithoutBlock, Expression};

pub enum AttributeKind {
    KwAbstract(KwAbstract),
    KwExport(KwExport),
    KwExtern(KwExtern),
    KwUnsafe(KwUnsafe),

    Path(SimplePath),
}

pub struct InnerAttr {
    hash_bang: HashBang,
    open_bracket: Bracket,
    attribute: AttributeKind,
    close_bracket: Bracket,
}

impl Expression for InnerAttr {}

impl<E> ExprWithoutBlock<E> for InnerAttr {}

impl Spanned for InnerAttr {
    fn span(&self) -> Span {
        let s1 = self.hash_bang.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)

        // let start_pos = self.hash_bang.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.hash_bang.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}

pub struct OuterAttr {
    hash: HashSign,
    open_bracket: Bracket,
    attribute: AttributeKind,
    close_bracket: Bracket,
}

impl Expression for OuterAttr {}

impl<E> ExprWithBlock<E> for OuterAttr {}

impl<E> ExprWithoutBlock<E> for OuterAttr {}

impl Spanned for OuterAttr {
    fn span(&self) -> Span {
        let s1 = self.hash.span();
        let s2 = self.close_bracket.span();

        Span::join(s1, s2)

        // let start_pos = self.hash.span().start();
        // let end_pos = self.close_bracket.span().end();
        // let source = self.hash.span().source();

        // let span = Span::new(source.as_str(), start_pos, end_pos);

        // span
    }
}
