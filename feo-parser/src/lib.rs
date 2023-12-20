use std::{fmt::Display, str::FromStr};

use feo_types::{
    span::Span, Comment, DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword,
    KeywordKind, Literal, PathExpression, Primitive, PrimitiveType, PuncKind, Punctuation,
    TypeAnnotation, TypeName,
};

mod lexer;
use lexer::{Lexer, Token};

mod literals;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};

mod parse;
use parse::{Parse, ParseVec};

impl<T> Parse<T> for CharLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.to_string().parse::<char>().map_err(|_| ())?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for StringLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let string_lit = Literal::new(content.to_string(), span);

        let token = Token::StringLit(StringLiteral(string_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for BoolLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.to_string().parse::<bool>().map_err(|_| ())?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for IntLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = i64::from_str_radix(&content.to_string(), 10 | 16).map_err(|_| ())?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for UIntLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = u64::from_str_radix(&content.to_string(), 10 | 16).map_err(|_| (()))?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for FloatLiteral
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let parsed = content.to_string().parse::<f64>().map_err(|_| (()))?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Identifier
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
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
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let keyword_kind = KeywordKind::from_str(&content.to_string())?;

        let keyword = Keyword::new(keyword_kind, span);

        let token = Token::Keyword(keyword);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Comment
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
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
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let doc_comment = DocComment::new(content.to_string(), span);

        let token = Token::DocComment(doc_comment);

        Ok(Some(token))
    }
}

impl<T> ParseVec<T> for PathExpression
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &Vec<T>, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let path: Vec<String> = content.into_iter().map(|t| t.to_string()).collect();

        let path_expr = PathExpression::new(path, span);

        let token = Token::Path(path_expr);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for Delimiter
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let delim_kind = match content.to_string().as_str() {
            "(" | ")" => Ok(DelimKind::Paren),
            "[" | "]" => Ok(DelimKind::Bracket),
            "{" | "}" => Ok(DelimKind::Brace),
            _ => Err(()),
        }?;

        let delim_orientation = match content.to_string().as_str() {
            "(" | "[" | "{" => Ok(DelimOrientation::Open),
            ")" | "]" | "}" => Ok(DelimOrientation::Close),
            _ => Err(()),
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
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let punc_kind = PuncKind::from_str(&content.to_string())?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
    }
}

impl<T> Parse<T> for TypeAnnotation
where
    T: 'static + Primitive + Display,
{
    fn parse(src: &str, content: &T, start: usize, end: usize) -> Result<Option<Token>, ()> {
        let span = Span::new(src, start, end);

        let type_name = TypeName::from_str(&content.to_string())?;

        let type_ann = TypeAnnotation::new(type_name, span);

        let token = Token::Type(type_ann);

        Ok(Some(token))
    }
}

impl TryFrom<Token> for Delimiter {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Delim(d) => Ok(d),
            _ => return Err(()),
        }
    }
}

pub fn lex() {
    // let filename = "path/to/your/file.txt"; // Change this to your file path
    // let source_code = std::fs::read_to_string(filename).expect("Error reading file");

    let source_code = r#"
        // line comment
        /// outer doc comment
        /* 
        block comment
        */

        /!
        module doc comment
        */

        let foo = "bar";

        let baz = -10;

        let foo = false;

        let bar: u32 = 10;

        let baz = 'a';
        "#;

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    if let Ok(t) = tokens {
        for token in t.tokens() {
            println!("{:?}", token);
        }
    } else {
        println!("Error tokenizing file");
    }
}
