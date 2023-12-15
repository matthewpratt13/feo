use feo_types::{
    Comment, Delimiter, DocComment, Identifier, PathExpression, Primitive, Punctuation,
    TypeAnnotation,
};

mod lexer;

mod literals;
use literals::{BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, UIntLiteral};

mod parse;
use parse::{Parse, ParseData /* ParseDigit */};

pub mod error;

// TODO:

impl Parse for Delimiter {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for CharLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for StringLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for BoolLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for IntLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        let start_pos = l.pos();
        let mut is_float = false;
        let mut is_hex = false;

        // Check for negative sign
        if self.current_char() == Some('-') {
            self.advance();
        }

        // // Check for hexadecimal prefix
        // if self.current_char() == Some('0')
            // && self.peek_next().map_or(false, |c| c == 'x' || c == 'X')
        // {
        //     self.advance(); // Skip '0'
        //     self.advance(); // Skip 'x'
        //     is_hex = true;
        // }

        // // Parse digits
        // while let Some(c) = self.current_char() {
        //     if c.is_digit(10) || (is_hex && c.is_digit(16)) {
        //         self.advance();
        //     } else if c == '.' && !is_float {
        //         self.advance();
        //         is_float = true;
        //     } else {
        //         break;
        //     }
        // }

        // let code = &self.input[start_pos..self.pos];

        // // Parse and return the appropriate token
        // if is_float {
        //     if let Ok(float_value) = code.parse::<f64>() {
        //         Token::Float(float_value)
        //     } else {
        //         self.log_error("Error parsing float");
        //         Token::Error(format!("Error parsing float: {}", code))
        //     }
        // } else if is_hex {
        //     if let Ok(uint_value) = BigUint::from_str_radix(code, 16) {
        //         Token::UInt(uint_value)
        //     } else {
        //         self.log_error("Error parsing hexadecimal integer");
        //         Token::Error(format!("Error parsing hexadecimal integer: {}", code))
        //     }
        // } else {
        //     if let Ok(int_value) = code.parse::<i64>() {
        //         Token::Int(int_value)
        //     } else {
        //         self.log_error("Error parsing integer");
        //         Token::Error(format!("Error parsing integer: {}", code))
        //     }
        // }
    }
}

impl Parse for UIntLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for FloatLiteral {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl Parse for Punctuation {
    fn parse(l: &mut lexer::Lexer) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for Comment
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for DocComment
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for Identifier
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for PathExpression
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}

impl<T> ParseData<T> for TypeAnnotation
where
    T: 'static + Primitive,
{
    fn parse(src: &str, input: T, start: usize, end: usize) -> Option<lexer::Token> {
        todo!()
    }
}
