use std::fmt::Display;

use feo_error::lex_error::LexErrorKind;
use feo_types::{
    span::Span, Comment, DelimKind, DelimOrientation, Delimiter, DocComment, Identifier, Keyword,
    KeywordKind, Literal, PathExpression, Primitive, PrimitiveType, PuncKind, Punctuation,
    TypeAnnotation, TypeName,
};

mod delimiter;

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
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let parsed = content
            .to_string()
            .parse::<char>()
            .map_err(|_| LexErrorKind::ParseCharError)?;

        let char_lit = Literal::new(parsed, span);

        let token = Token::CharLit(CharLiteral(char_lit));

        Ok(Some(token))
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
    ) -> Result<Option<Token>, LexErrorKind> {
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
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let parsed = content
            .to_string()
            .parse::<bool>()
            .map_err(|_| LexErrorKind::ParseBoolError)?;

        let bool_lit = Literal::new(parsed, span);

        let token = Token::BoolLit(BoolLiteral(bool_lit));

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
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let parsed = i64::from_str_radix(&content.to_string(), 10 | 16)
            .map_err(|_| LexErrorKind::ParseIntError)?;

        let int_lit = Literal::new(parsed, span);

        let token = Token::IntLit(IntLiteral(int_lit));

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
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let parsed = u64::from_str_radix(&content.to_string(), 10 | 16)
            .map_err(|_| LexErrorKind::ParseIntError)?;

        let uint_lit = Literal::new(parsed, span);

        let token = Token::UIntLit(UIntLiteral(uint_lit));

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
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let parsed = content
            .to_string()
            .parse::<f64>()
            .map_err(|_| LexErrorKind::ParseFloatError)?;

        let float_lit = Literal::new(parsed, span);

        let token = Token::FloatLit(FloatLiteral(float_lit));

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
    ) -> Result<Option<Token>, LexErrorKind> {
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
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let keyword_kind = match content.to_string().as_str() {
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
            _ => Err(LexErrorKind::InvalidKeyword),
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
    ) -> Result<Option<Token>, LexErrorKind> {
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
    ) -> Result<Option<Token>, LexErrorKind> {
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
    fn parse(
        src: &str,
        content: &Vec<T>,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind> {
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
    fn parse(
        src: &str,
        content: &T,
        start: usize,
        end: usize,
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let delim_kind = match content.to_string().as_str() {
            "(" | ")" => Ok(DelimKind::Paren),
            "[" | "]" => Ok(DelimKind::Bracket),
            "{" | "}" => Ok(DelimKind::Brace),
            _ => Err(LexErrorKind::InvalidDelimiter),
        }?;

        let delim_orientation = match content.to_string().as_str() {
            "(" | "[" | "{" => Ok(DelimOrientation::Open),
            ")" | "]" | "}" => Ok(DelimOrientation::Close),
            _ => Err(LexErrorKind::InvalidDelimiter),
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
    ) -> Result<Option<Token>, LexErrorKind> {
        let span = Span::new(src, start, end);

        let punc_kind = match content.to_string().as_str() {
            ":" => Ok(PuncKind::Colon),
            ";" => Ok(PuncKind::Semicolon),
            "," => Ok(PuncKind::Comma),
            "." => Ok(PuncKind::FullStop),
            "_" => Ok(PuncKind::Underscore),
            "::" => Ok(PuncKind::DoubleColon),
            ".." => Ok(PuncKind::DoubleFullStop),
            "//" => Ok(PuncKind::DoubleSlash),
            "///" => Ok(PuncKind::TripleSlash),
            "/*" => Ok(PuncKind::SlashAsterisk),
            "/!" => Ok(PuncKind::SlashBang),
            "*/" => Ok(PuncKind::AsteriskSlash),
            "!" => Ok(PuncKind::Bang),
            "#" => Ok(PuncKind::Hash),
            "$" => Ok(PuncKind::DollarSign),
            "%" => Ok(PuncKind::Percent),
            "&" => Ok(PuncKind::Ampersand),
            "*" => Ok(PuncKind::Asterisk),
            "+" => Ok(PuncKind::Plus),
            "-" => Ok(PuncKind::Minus),
            "/" => Ok(PuncKind::ForwardSlash),
            "<" => Ok(PuncKind::LessThan),
            "=" => Ok(PuncKind::Equals),
            ">" => Ok(PuncKind::GreaterThan),
            "?" => Ok(PuncKind::QuestionMark),
            "@" => Ok(PuncKind::AtSign),
            "^" => Ok(PuncKind::Caret),
            "`" => Ok(PuncKind::BackTick),
            "|" => Ok(PuncKind::Pipe),
            "~" => Ok(PuncKind::Tilde),
            "!=" => Ok(PuncKind::BangEquals),
            "%=" => Ok(PuncKind::PercentEquals),
            "*=" => Ok(PuncKind::AsteriskEquals),
            "**" => Ok(PuncKind::DoubleAsterisk),
            "&&" => Ok(PuncKind::DoubleAmpersand),
            "+=" => Ok(PuncKind::PlusEquals),
            "-=" => Ok(PuncKind::MinusEquals),
            "/=" => Ok(PuncKind::ForwardSlashEquals),
            "<=" => Ok(PuncKind::LessThanEquals),
            "==" => Ok(PuncKind::DoubleEquals),
            ">=" => Ok(PuncKind::GreaterThanEquals),
            "->" => Ok(PuncKind::ThinArrow),
            "=>" => Ok(PuncKind::FatArrow),
            "||" => Ok(PuncKind::DoublePipe),
            "\n" => Ok(PuncKind::Newline),
            "\r" => Ok(PuncKind::Return),
            "\t" => Ok(PuncKind::Tab),
            "\\" => Ok(PuncKind::Backslash),
            "\0" => Ok(PuncKind::Null),
            "\'" => Ok(PuncKind::SingleQuote),
            "\"" => Ok(PuncKind::DoubleQuote),
            _ => Err(LexErrorKind::InvalidPunctuation),
        }?;

        let punc = Punctuation::new(punc_kind, span);

        let token = Token::Punc(punc);

        Ok(Some(token))
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
    ) -> Result<Option<Token>, LexErrorKind> {
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

// TODO: return `LexError`
pub fn lex() -> Result<(), ()> {
    let filename = "path/to/your/file.txt"; // Change this to your file path
    let source_code = std::fs::read_to_string(filename).expect("Error reading file");

    let mut lexer = Lexer::new(&source_code);
    let tokens = lexer.tokenize();

    if let Ok(t) = tokens {
        for token in t.tokens() {
            println!("{:?}", token);
        }
    }

    Ok(())
}
