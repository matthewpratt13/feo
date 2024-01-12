use crate::{
    identifier::Identifier,
    item::Parenthesis,
    keyword::KeywordKind,
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    path::SimplePath,
};

use self::{slice_patt::SlicePatt, struct_patt::StructPatt};

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

mod struct_patt {
    use crate::{
        expression::Attribute,
        identifier::Identifier,
        item::{Brace, Colon, Comma},
        keyword::KeywordKind,
        path::SimplePath,
    };

    use super::Pattern;

    pub enum StructPattKind {
        WithoutBody(StructWithoutBody),
        WithBody(StructWithBody),
    }

    pub struct StructWithoutBody {
        kw_ref_opt: Option<KeywordKind>,
        kw_mut_opt: Option<KeywordKind>,
        name: Identifier,
    }

    pub struct StructWithBody {
        name: Identifier,
        colon: Colon,
        pattern: Box<Pattern>,
    }

    pub struct StructPatt {
        path: SimplePath,
        open_brace: Brace,
        struct_patt_fields_opt: Option<StructPattFields>,
        trailing_comma_opt: Option<Comma>,
        close_brace: Brace,
    }

    pub struct StructPattFields {
        first_field: StructPattField,
        subsequent_fields: Vec<(Comma, StructPattField)>,
        trailing_comma_opt: Option<Comma>,
    }

    pub struct StructPattField {
        attribute: Attribute,
        struct_pattern_kind: StructPattKind,
    }
}

mod tuple_struct_item {
    use crate::{
        item::{Comma, Parenthesis},
        path::SimplePath,
    };

    use super::Pattern;

    pub struct TupleStructPatt {
        path: SimplePath,
        open_parenthesis: Parenthesis,
        tuple_struct_items_opt: Option<TupleStructElements>,
        close_parenthesis: Parenthesis,
    }

    pub struct TupleStructElements {
        first_pattern: Pattern,
        subsequent_patterns: Vec<(Comma, Pattern)>,
        trailing_comma_opt: Option<Comma>,
    }
}

pub struct TuplePatt {}

pub struct TuplePattItems {}

pub struct WildcardPatt {}
