pub mod comment;
pub mod delimiter;
pub mod doc_comment;
pub mod expression;
pub mod identifier;
pub mod item;
pub mod keyword;
pub mod literals;
pub mod path;
pub mod pattern;
pub mod program;
pub mod punctuation;
pub mod statement;
pub mod token;
pub mod ty;
pub mod type_annotation;

mod type_utils {
    use crate::delimiter::Delimiter;
    use crate::punctuation::Punctuation;

    pub type Asterisk = Punctuation;
    pub type Bang = Punctuation;
    pub type Colon = Punctuation;
    pub type Comma = Punctuation;
    pub type DblColon = Punctuation;
    pub type DblDot = Punctuation;
    pub type DblPipe = Punctuation;
    pub type Dot = Punctuation;
    pub type DotDotEquals = Punctuation;
    pub type Equals = Punctuation;
    pub type FatArrow = Punctuation;
    pub type HashBang = Punctuation;
    pub type HashSign = Punctuation;
    pub type OpArithmeticOrLogical = Punctuation;
    pub type OpBool = Punctuation;
    pub type OpComparison = Punctuation;
    pub type Pipe = Punctuation;
    pub type Plus = Punctuation;
    pub type QuestionMark = Punctuation;
    pub type Semicolon = Punctuation;
    pub type ThinArrow = Punctuation;

    pub type Brace = Delimiter;
    pub type Bracket = Delimiter;
    pub type Parenthesis = Delimiter;
}

mod primitive_wrapper {
    use std::fmt;

    use feo_types::span::{Span, Spanned};
    use feo_types::U256;

    use crate::expression::{Constant, ExprWithoutBlock, Expression};
    use crate::pattern::{Pattern, RangePattBound};
    use crate::statement::Statement;

    pub trait LiteralExpr<E>
    where
        Self: Sized + Constant + ExprWithoutBlock<E>,
    {
    }

    pub trait LiteralPatt
    where
        Self: Sized + 'static + Pattern,
    {
    }

    pub struct CharValue(char);

    impl<E> LiteralExpr<E> for CharValue {}

    impl Expression for CharValue {}

    impl<E> ExprWithoutBlock<E> for CharValue {}

    impl Statement for CharValue {}

    impl Constant for CharValue {}

    impl LiteralPatt for CharValue {}

    impl Pattern for CharValue {}

    impl RangePattBound for CharValue {}

    impl Spanned for CharValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for CharValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct StrValue(&'static str);

    impl<E> LiteralExpr<E> for StrValue {}

    impl Expression for StrValue {}

    impl<E> ExprWithoutBlock<E> for StrValue {}

    impl Statement for StrValue {}

    impl Constant for StrValue {}

    impl LiteralPatt for StrValue {}

    impl Pattern for StrValue {}

    impl Spanned for StrValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for StrValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct IntValue(i64);

    impl<E> LiteralExpr<E> for IntValue {}

    impl Expression for IntValue {}

    impl<E> ExprWithoutBlock<E> for IntValue {}

    impl Statement for IntValue {}

    impl Constant for IntValue {}

    impl LiteralPatt for IntValue {}

    impl Pattern for IntValue {}

    impl RangePattBound for IntValue {}

    impl Spanned for IntValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for IntValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct UIntValue(u64);

    // impl UIntValue {
    //     fn trim_leading_zeros(self) -> Self {
    //         let uint_string = format!("{}", self.0);
    //         let stripped = uint_string.as_str().trim_start_matches('0');
    //         let new_uint = u64::from_str_radix(stripped, 10).expect("Unable to parse str to u64");

    //         Self(new_uint)
    //     }
    // }

    impl<E> LiteralExpr<E> for UIntValue {}

    impl Expression for UIntValue {}

    impl<E> ExprWithoutBlock<E> for UIntValue {}

    impl Statement for UIntValue {}

    impl Constant for UIntValue {}

    impl LiteralPatt for UIntValue {}

    impl Pattern for UIntValue {}

    impl RangePattBound for UIntValue {}

    impl Spanned for UIntValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for UIntValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct U256Value(U256);

    impl<E> LiteralExpr<E> for U256Value {}

    impl Expression for U256Value {}

    impl<E> ExprWithoutBlock<E> for U256Value {}

    impl Statement for U256Value {}

    impl Constant for U256Value {}

    impl LiteralPatt for U256Value {}

    impl Pattern for U256Value {}

    impl RangePattBound for U256Value {}

    impl Spanned for U256Value {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for U256Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct FloatValue(f64);

    impl<E> LiteralExpr<E> for FloatValue {}

    impl Expression for FloatValue {}

    impl<E> ExprWithoutBlock<E> for FloatValue {}

    impl Statement for FloatValue {}

    impl Constant for FloatValue {}

    impl LiteralPatt for FloatValue {}

    impl Pattern for FloatValue {}

    impl RangePattBound for FloatValue {}

    impl Spanned for FloatValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for FloatValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct Bytes32Value(&'static [u8; 32]);

    impl<E> LiteralExpr<E> for Bytes32Value {}

    impl Expression for Bytes32Value {}

    impl<E> ExprWithoutBlock<E> for Bytes32Value {}

    impl Statement for Bytes32Value {}

    impl Constant for Bytes32Value {}

    impl LiteralPatt for Bytes32Value {}

    impl Pattern for Bytes32Value {}

    impl Spanned for Bytes32Value {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for Bytes32Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    pub struct BoolValue(bool);

    impl<E> LiteralExpr<E> for BoolValue {}

    impl Expression for BoolValue {}

    impl<E> ExprWithoutBlock<E> for BoolValue {}

    impl Statement for BoolValue {}

    impl Constant for BoolValue {}

    impl LiteralPatt for BoolValue {}

    impl Pattern for BoolValue {}

    impl Spanned for BoolValue {
        fn span(&self) -> Span {
            Span::default()
        }
    }

    impl fmt::Display for BoolValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
}
