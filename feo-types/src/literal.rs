pub trait Literal {}

pub trait FeoLiteral<L: Literal>
where
    Self: Sized,
{
    fn new(input: L) -> Self;
}

pub struct CharLiteral {}
pub struct StringLiteral {}
pub struct IntLiteral {}
pub struct FloatLiteral {}
pub struct BoolLiteral {}
