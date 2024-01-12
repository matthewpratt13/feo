use crate::{
    identifier::Identifier,
    item::Parenthesis,
    keyword::KeywordKind,
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    path::SimplePath,
};

use self::slice_patt::SlicePatt;

pub enum Pattern {
    Literal(LiteralPatt),
    Grouped(GroupedPatt),
    Identifier(IdentifierPatt),
    Path(SimplePath),
    Reference(ReferencePatt),
    Slice(SlicePatt),
    Struct(StructPatt),
    Tuple(TuplePatt),
    TupleStruct(TupleStructPatt),
    Wildcard(WildcardPatt),
}

pub enum LiteralPatt {
    Char(CharLiteral),
    String(StringLiteral),
    Int(IntLiteral),
    UInt(UIntLiteral),
    U256(U256Literal),
    Float(FloatLiteral),
    Bool(BoolLiteral),
}

pub struct GroupedPatt {
    open_parenthesis: Parenthesis,
    pattern: Box<Pattern>,
    close_parenthesis: Parenthesis,
}

pub struct IdentifierPatt {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    name: Identifier,
}

pub struct ReferencePatt {
    kw_ref_opt: Option<KeywordKind>,
    kw_mut_opt: Option<KeywordKind>,
    name: Box<Pattern>,
}

mod slice_patt {
    use crate::item::{Bracket, Comma};

    use super::Pattern;

    pub struct SlicePatt {
        open_bracket: Bracket,
        slice_pattern_items_opt: Option<SlicePattItems>,
        close_bracket: Bracket,
    }

    pub struct SlicePattItems {
        first_pattern: Box<Pattern>,
        subsequent_patterns: Vec<(Comma, Pattern)>,
        trailing_comma_opt: Option<Comma>,
    }
}

pub struct StructPatt {}

pub struct StructPattFields {}

pub struct StructPattField {}

pub struct TuplePatt {}

pub struct TuplePattItems {}

pub struct TupleStructPatt {}

pub struct TupleStructItems {}

pub struct WildcardPatt {}
