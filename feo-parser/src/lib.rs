use std::fmt::Display;

use error::ParserError;
use feo_types::{
    span::Span, Comment, DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword,
    KeywordKind, Literal, PathExpression, Primitive, PrimitiveType, Punctuation, TypeAnnotation,
    TypeName,
};

mod lexer;

mod literals;
use lexer::Token;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};
use parse::Parse;

mod parse;

pub mod error;

// TODO:

impl<T> Parse<T> for CharLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for StringLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for BoolLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let parsed = content.to_string().parse::<char>()?;

        let lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for IntLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let parsed = i64::from_str_radix(&content.to_string(), 10 | 16)?;

        let lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for UIntLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let parsed = u64::from_str_radix(&content.to_string(), 10 | 16)?;

        let lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for FloatLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let parsed = content.to_string().parse::<f64>()?;

        let lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Identifier
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let iden = Identifier::new(content.to_string(), span);

        let token = Token::Iden(iden);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Keyword
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let keyword_kind = match content.to_string().as_str() {
            "abstract" => Ok(KeywordKind::AbstractKw),
            "as" => Ok(KeywordKind::AsKw),
            "break" => Ok(KeywordKind::BreakKw),
            "const" => Ok(KeywordKind::ConstKw),
            "continue" => Ok(KeywordKind::ContinueKw),
            "deref" => Ok(KeywordKind::DerefKw),
            "else" => Ok(KeywordKind::ElseKw),
            "enum" => Ok(KeywordKind::EnumKw),
            "for" => Ok(KeywordKind::ForKw),
            "func" => Ok(KeywordKind::FuncKw),
            "if" => Ok(KeywordKind::IfKw),
            "impl" => Ok(KeywordKind::ImplKw),
            "import" => Ok(KeywordKind::ImportKw),
            "in" => Ok(KeywordKind::InKw),
            "let" => Ok(KeywordKind::LetKw),
            "library" => Ok(KeywordKind::LibraryKw),
            "loop" => Ok(KeywordKind::LoopKw),
            "match" => Ok(KeywordKind::MatchKw),
            "mod" => Ok(KeywordKind::ModKw),
            "mut" => Ok(KeywordKind::MutKw),
            "pub" => Ok(KeywordKind::PubKw),
            "ref" => Ok(KeywordKind::RefKw),
            "return" => Ok(KeywordKind::ReturnKw),
            "self" => Ok(KeywordKind::SelfKw),
            "static" => Ok(KeywordKind::StaticKw),
            "struct" => Ok(KeywordKind::StructKw),
            "super" => Ok(KeywordKind::SuperKw),
            "trait" => Ok(KeywordKind::TraitKw),
            "type" => Ok(KeywordKind::TypeKw),
            "while" => Ok(KeywordKind::WhileKw),
            _ => Err(ParserError::InvalidKeyword),
        }?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Comment
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let comment = Comment::new(content.to_string(), span);

        let token = Token::Comment(comment);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for DocComment
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

// TODO: find a way to properly implement this:
impl<T> Parse<T> for PathExpression
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let path_expr = PathExpression::new(content, span);

        let token = Token::Path(path_expr);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Delimiter
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let delim_kind = match content.to_string().as_str() {
            "(" | ")" => Ok(DelimKind::Paren),
            "[" | "]" => Ok(DelimKind::Bracket),
            "{" | "}" => Ok(DelimKind::Brace),
            _ => Err(ParserError::InvalidDelimiter),
        }?;

        let delim_orientation = match content.to_string().as_str() {
            "(" | "[" | "{" => Ok(DelimOrientation::Open),
            ")" | "]" | "}" => Ok(DelimOrientation::Close),
            _ => Err(ParserError::InvalidDelimiter),
        }?;

        let delim = Delimiter::new(delim_kind, delim_orientation, span);

        let token = Token::Delim(delim);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Punctuation
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        todo!()
    }
}

impl<T> Parse<T> for TypeAnnotation
where
    T: 'static + Primitive + Display,
{
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, ParserError> {
        let span = Span::new(src, start, end);

        let type_name = match content.to_string().as_str() {
            "bool" => Ok(TypeName::BoolType),
            "char" => Ok(TypeName::CharType),
            "f32" => Ok(TypeName::F32Type),
            "f64" => Ok(TypeName::F64Type),
            "i32" => Ok(TypeName::I32Type),
            "i64" => Ok(TypeName::I64Type),
            "String" => Ok(TypeName::StringType),
            "u8" => Ok(TypeName::U8Type),
            "u16" => Ok(TypeName::U16Type),
            "u32" => Ok(TypeName::U32Type),
            "u64" => Ok(TypeName::U32Type),
            _ => Ok(TypeName::CustomType(content.to_string())),
        }?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}
