use core::iter::Peekable;
use std::sync::Arc;

use feo_ast::{
    comment::Comment,
    delimiter::Delimiter,
    doc_comment::DocComment,
    identifier::Identifier,
    keyword::Keyword,
    literals::{
        BoolLiteral, CharLiteral, FloatLiteral, IntLiteral, StringLiteral, U256Literal, UIntLiteral,
    },
    punctuation::Punctuation,
    token::{Token, TokenStream, Tokenize},
    type_annotation::{TypeAnnKind, TypeAnnotation},
};

use feo_error::{
    error::CompilerError,
    handler::{ErrorEmitted, Handler},
    lex_error::{LexError, LexErrorKind},
};
use feo_types::span::Position;

struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    peekable_chars: Peekable<core::str::Chars<'a>>,
    handler: Handler,
}

#[allow(dead_code)]
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, handler: Handler) -> Self {
        Self {
            input,
            pos: 0,
            peekable_chars: input.chars().peekable(),
            handler,
        }
    }

    // progress through the source code so that the lexer can continue to process chars
    fn advance(&mut self) -> Option<char> {
        // update the lexer's position or other internal state if needed
        self.pos += 1;
        // move to the next char in the iterator
        self.peekable_chars.next()
    }

    // return the current char at the lexer's current position without advancing the pos
    fn current_char(&mut self) -> Option<char> {
        self.peekable_chars.peek().cloned()
    }

    // return the next char in the input stream (i.e., peek ahead by one char)
    fn peek_next(&self) -> Option<char> {
        // create a clone of the iterator and advance this cloned iterator
        let mut cloned_iter = self.peekable_chars.clone();
        cloned_iter.next();

        // peek at the next char from the original iterator
        cloned_iter.peek().cloned()
    }

    // advance the lexer's pos past any whitespace chars in the input stream
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                // if the current char is whitespace, advance to the next char
                self.advance();
            } else {
                // if the current char is not whitespace, break out of the loop
                break;
            }
        }
    }

    // log and store information about an error encountered during the lexing process
    fn log_error(&self, error_kind: LexErrorKind) -> ErrorEmitted {
        let err = LexError {
            error_kind,
            position: Position::new(self.input.trim_start_matches('\n'), self.pos),
        };

        self.handler.emit_err(CompilerError::Lex(err))
    }

    // main lexer function
    // return a stream of tokens, parsed and tokenized from an input stream (i.e., source code)
    pub fn lex(&mut self) -> Result<TokenStream<Token>, ErrorEmitted> {
        let mut tokens: Vec<Option<Token>> = Vec::new();

        let mut num_open_delimiters: usize = 0; // to check for unclosed delimiters

        while let Some(c) = self.current_char() {
            let start_pos = self.pos;

            match c {
                _ if c.is_whitespace() => {
                    self.skip_whitespace();
                }

                _ if c == '/' && self.peek_next() == Some('/') || self.peek_next() == Some('*') => {
                    self.advance(); // skip first '/'

                    match self.current_char() {
                        Some('/') => {
                            self.advance(); // skip second '/'

                            // doc comment
                            if self.current_char() == Some('/') {
                                self.advance(); // skip third '/'
                                self.skip_whitespace();

                                let start_pos = self.pos; // start reading after the three '/'

                                while let Some(c) = self.current_char() {
                                    if c == '\n' {
                                        break;
                                    } else {
                                        self.advance();
                                    }
                                }

                                let raw_content = self.input[start_pos..self.pos].to_string();

                                let doc_comment_content = Arc::new(&raw_content);

                                let doc_comment = DocComment::tokenize(
                                    &self.input,
                                    &doc_comment_content,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(doc_comment);
                            } else {
                                while let Some(c) = self.current_char() {
                                    if c == '\n' {
                                        break;
                                    } else {
                                        self.advance();
                                    }
                                }

                                let raw_data = self.input[start_pos..self.pos].to_string();

                                let comment_data = Arc::new(&raw_data);

                                let comment = Comment::tokenize(
                                    &self.input,
                                    &comment_data,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(comment);
                            }
                        }

                        Some('*') => {
                            self.advance(); // skip '*'

                            while let Some(c) = self.current_char() {
                                if c == '*' {
                                    self.advance(); // skip closing '*'
                                    self.advance(); // skip closing '/'
                                    break;
                                } else {
                                    self.advance();
                                }
                            }

                            let raw_data = self.input[start_pos..self.pos].to_string();

                            let comment_data = Arc::new(&raw_data);

                            let comment = Comment::tokenize(
                                &self.input,
                                &comment_data,
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(comment);
                        }

                        Some(_) | None => (),
                    }
                }

                // identifiers and keywords
                'A'..='Z' | 'a'..='z' | '_' => {
                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        // cannot start with, but can contain numbers (`is_ascii_alphanumeric()`)
                        if c.is_ascii_alphanumeric() || c == '_' {
                            buf.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    if buf == "true" || buf == "false" {
                        let bool_lit = BoolLiteral::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(bool_lit);
                        continue;
                    }

                    if is_built_in_type_annotation(&buf) {
                        let type_annotation = TypeAnnotation::tokenize(
                            &self.input,
                            &buf,
                            start_pos,
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(type_annotation);
                    } else if is_keyword(&buf) {
                        let keyword = Keyword::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(keyword);
                    } else {
                        let iden = Identifier::tokenize(
                            &self.input,
                            &buf,
                            start_pos, // global `start_pos`
                            start_pos + buf.len(),
                            &mut self.handler,
                        )?;

                        tokens.push(iden);
                    }
                }

                '(' | '[' | '{' => {
                    self.advance(); // skip opening delimiter

                    match c {
                        '(' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                "(",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }

                        '[' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                "[",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }

                        '{' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                "{",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };

                    num_open_delimiters += 1;
                }

                ')' | ']' | '}' => {
                    self.advance(); // skip closing delimiter (advance counter for correct end pos)

                    match c {
                        ')' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                ")",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }

                        ']' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                "]",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }

                        '}' => {
                            let delim = Delimiter::tokenize(
                                &self.input,
                                "}",
                                start_pos,
                                self.pos,
                                &mut self.handler,
                            )?;

                            tokens.push(delim);
                        }
                        _ => unreachable!(),
                    };

                    num_open_delimiters -= 1;
                }

                '"' => {
                    // `start_pos` is global `start_pos` (above)
                    self.advance(); // skip opening '"' (double quote)

                    let mut string_literal_open = true; // to check for unclosed quotes

                    let mut buf = String::new();

                    while let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                if let Some(esc_c) = self.current_char() {
                                    self.advance(); // return escaped char

                                    match esc_c {
                                        'n' => buf.push('\n'),
                                        'r' => buf.push('\r'),
                                        't' => buf.push('\t'),
                                        '\\' => buf.push('\\'),
                                        '0' => buf.push('\0'),
                                        '\'' => buf.push('\''),
                                        '"' => buf.push('"'),
                                        _ => {
                                            return Err(
                                                self.log_error(LexErrorKind::InvalidEscapeSequence)
                                            )
                                        }
                                    };
                                } else {
                                    // escape sequence is expected, but the input has ended
                                    return Err(
                                        self.log_error(LexErrorKind::ExpectedEscapeSequence)
                                    );
                                }
                            }

                            '"' => {
                                self.advance(); // skip closing '"'

                                string_literal_open = false;

                                let string_lit = StringLiteral::tokenize(
                                    &self.input,
                                    &buf,
                                    start_pos,
                                    self.pos,
                                    &mut self.handler,
                                )?;

                                tokens.push(string_lit);
                                break;
                            }

                            _ => {
                                buf.push(c);
                                self.advance();
                            }
                        }
                    }

                    if string_literal_open {
                        return Err(self.log_error(LexErrorKind::UnclosedStringLiteral));
                    }
                }

                '\'' => {
                    self.advance(); // skip opening '\'' (single quote)

                    let start_pos = self.pos; // start reading input after opening quote

                    if let Some(c) = self.current_char() {
                        match c {
                            '\\' => {
                                self.advance(); // skip '\'

                                let esc_char_lit = match self.current_char() {
                                    Some('n') => CharLiteral::tokenize(
                                        &self.input,
                                        "\n",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('r') => CharLiteral::tokenize(
                                        &self.input,
                                        "\r",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('t') => CharLiteral::tokenize(
                                        &self.input,
                                        "\t",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('\\') => CharLiteral::tokenize(
                                        &self.input,
                                        "\\",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('0') => CharLiteral::tokenize(
                                        &self.input,
                                        "\0",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('\'') => CharLiteral::tokenize(
                                        &self.input,
                                        "'",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    Some('"') => CharLiteral::tokenize(
                                        &self.input,
                                        "\"",
                                        start_pos,
                                        self.pos + 1,
                                        &mut self.handler,
                                    )?,

                                    _ => {
                                        return Err(
                                            self.log_error(LexErrorKind::InvalidEscapeSequence)
                                        )?
                                    }
                                };

                                tokens.push(esc_char_lit);
                                self.advance(); // skip second char

                                if self.current_char() != Some('\'') {
                                    return Err(
                                        self.log_error(LexErrorKind::ExpectedClosingSingleQuote)
                                    );
                                }

                                self.advance(); // skip closing '\'' (single quote)
                            }

                            '\'' => {
                                return Err(self.log_error(LexErrorKind::EmptyCharLiteral));
                            }

                            _ => {
                                if c == ' ' {
                                    return Err(self.log_error(LexErrorKind::InvalidCharLiteral));
                                }

                                self.advance(); // return next (regular) char

                                if self.current_char() == Some('\'') {
                                    let char_lit = CharLiteral::tokenize(
                                        &self.input,
                                        &c.to_string(),
                                        start_pos,
                                        self.pos,
                                        &mut self.handler,
                                    )?;

                                    tokens.push(char_lit);
                                    self.advance(); // skip closing '\'' (single quote)
                                } else {
                                    return Err(self.log_error(LexErrorKind::InvalidCharLiteral));
                                }
                            }
                        }
                    } else {
                        return Err(self.log_error(LexErrorKind::ExpectedCharLiteral));
                    }
                }

                // check for hexadecimal prefix
                _ if c == '0' && self.peek_next() == Some('x') => {
                    // `start_pos` is global `start_pos` (above)

                    let mut is_u256 = false;

                    let i = tokens.len() - 2; // go backwards: skip the '=', return the 'type_ann'

                    let t = tokens
                        .get(i)
                        .expect("Token not found")
                        .clone()
                        .expect("Token not found");

                    if let Ok(ta) = TypeAnnKind::try_from(t)
                        .map_err(|_| self.log_error(LexErrorKind::TypeAnnotationError))
                    {
                        if ta == TypeAnnKind::TypeAnnU256 {
                            is_u256 = true;
                        }
                    }

                    self.advance(); // skip '0'
                    self.advance(); // skip 'x'

                    while let Some(c) = self.current_char() {
                        if c.is_digit(16) || c == '_' {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    if is_u256 {
                        let u256_lit = U256Literal::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(u256_lit);
                    } else {
                        let uint_lit = UIntLiteral::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(uint_lit);
                    }
                }

                _ if c.is_digit(10)
                    || (c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10))) =>
                {
                    let mut is_negative = false;

                    if c == '-' && self.peek_next().is_some_and(|c| c.is_digit(10)) {
                        is_negative = true;
                        self.advance(); // skip '-'
                    }

                    // go back and read from previous char ('-') if neg, else read from current pos
                    let start_pos = if is_negative { self.pos - 1 } else { self.pos };

                    let mut is_float = false;

                    while let Some(c) = self.current_char() {
                        if c.is_digit(10) || c == '_' {
                            self.advance();
                        } else if c == '.' && !is_float {
                            self.advance();
                            is_float = true;
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let num_content = Arc::new(&data);

                    if is_float {
                        let float_lit = FloatLiteral::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(float_lit);
                        continue;
                    }

                    if is_negative {
                        let int_lit = IntLiteral::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(int_lit);
                    } else {
                        let uint_lit = UIntLiteral::tokenize(
                            &self.input,
                            &num_content,
                            start_pos,
                            self.pos,
                            &mut self.handler,
                        )?;

                        tokens.push(uint_lit);
                    }
                }

                '!' | '#'..='&' | '*'..='/' | ':'..='@' | '\\' | '^' | '`' | '|' => {
                    while let Some(c) = self.current_char() {
                        if c.is_ascii_punctuation() {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    let data = self.input[start_pos..self.pos].to_string();

                    let punc_content = Arc::new(&data);

                    let punc = Punctuation::tokenize(
                        &self.input,
                        &punc_content,
                        start_pos,
                        self.pos,
                        &mut self.handler,
                    )?;

                    tokens.push(punc);
                }

                _ => return Err(self.log_error(LexErrorKind::InvalidChar(c))),
            }
        }

        if num_open_delimiters > 0 {
            return Err(self.log_error(LexErrorKind::UnclosedDelimiters));
        }

        let stream = TokenStream::new(&self.input, tokens, 0, self.pos);

        Ok(stream)
    }

    pub fn errors(&self) -> Vec<CompilerError> {
        self.handler.clone().get_inner().0
    }
}

fn is_built_in_type_annotation(iden: &str) -> bool {
    [
        "char", "String", "bool", "i32", "i64", "u8", "u16", "u32", "u64", "u256", "f32", "f64",
        "Vec",
    ]
    .contains(&iden)
}

fn is_keyword(iden: &str) -> bool {
    [
        "abstract",
        "as",
        "break",
        "const",
        "continue",
        "contract",
        "deref",
        "else",
        "enum",
        "extern",
        "for",
        "func",
        "if",
        "impl",
        "import",
        "in",
        "interface",
        "let",
        "library",
        "loop",
        "match",
        "mod",
        "mut",
        "program",
        "pub",
        "ref",
        "return",
        "script",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "type",
        "while",
    ]
    .contains(&iden)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex() {
        // let source_code = r#"
        // // line comment
        // /*
        // block comment
        // */
        // /// doc comment

        // struct Foo {
        //     a: String, // trailing comment
        //     b: i32,
        //     c: char,
        //     d: bool
        // }

        // impl Foo {
        //     pub func new() -> Foo {
        //         let vec = [0xBEEF_ABCD, 2_000_000, 3, 4];
        //         let mut new_vec: Vec<f64> = [];
        //         let big_uint: u256 = 0x0123_4567_89AB_CDEF

        //         if foo < 0 {
        //             print("{}", foo);
        //         } else {
        //             print("{}", foo);
        //         }

        //         for i in vec {
        //             new_vec.push(i / 1.0);
        //         }

        //         return Foo {
        //             a: "foo",
        //             b: -123_456,
        //             c: '\'',
        //             d: false
        //         };
        //     }
        // }
        // "#;

        let source_code = r#"
        // line comment

        /*
        block comment
        */

        /// doc comment
        
        library;
        
        func hello_world() {
            print!("hello world");
        }
        
        program;

        script Main {
            import some_module::SomeContract;
            import another_public_module::SomeAbstractContract;

            func main() {
                greater_than(1, 2);

                let hello = SomeContract::bar();
                let world = SomeAbstractContract::bar();

                print!("{} {}", hello, world);

                some_library::hello_world();
            }

            func greater_than(arg1: u256, arg2: u256) {
                if arg1 > arg2 {
                    print!("{} is greater than {}", arg1, arg2);
                } else if arg1 == arg2 {
                    print!("{} is equal to {}", arg1, arg2);
                } else {
                    print!("{} is less than {}", arg1, arg2);
                }
            }
        }

        
        pub mod some_public_module {

            pub trait SomeTrait {
                func bar() -> String; 
            }
        }

        pub mod another_public_module {
            import super::some_public_module::SomeTrait;

            pub enum Colour {
                Red,
                Green, 
                Blue
            }

            abstract contract SomeAbstractContract {
                pub interface {
                    func colour(arg: char) -> Colour?;
                }

                impl SomeTrait for SomeAbstractContract {
                    pub func bar() -> String {
                        return "world"
                    }
                }
            }
        }

        mod some_module {
            import super::some_public_module::SomeTrait;
            import super::another_public_module::{SomeAbstractContract, Colour};

            struct Foo {
                field1: String,
                field2: char,
                field3: u256,
                field4: Vec<f64>,
                field5: i64,
                field6: bool
            }

            contract SomeContract {
                pub storage {
                    pub const ADDRESS: Identity = Identity::Contract(ContractId::from(U256::ZERO));
                    pub static OWNER: Identity = Identity::User(UserId::from(msg.sender()));
                    static BALANCE: u64 = 0;
                }

                interface {
                    func foo() -> Foo;
                }

                impl SomeContract {
                    func foo() -> Foo {
                        let vec: Vec<u64> = [1, 2, 3, 4];
                        let mut new_vec: Vec<f64> = [];

                        for num in vec {
                            new_vec.push(num as f64);
                        }

                        new_vec.push(5.0);

                        return Foo {
                            field1: "foo",
                            field2: '\'',
                            field3: 0x0123_4567_89AB_CDEF,
                            field4: new_vec,
                            field5: -1234,
                            field6: true
                        }
                    }
                }

                impl SomeAbstractContract for SomeContract {
                    func colour(arg: char) -> Colour? {
                        return match arg {
                            'r' => Some(Colour::Red),
                            'g' => Some(Colour::Green)
                            'b' => Some(Colour::Blue),
                            _ => None
                        }
                    }
                }

                impl SomeTrait for SomeContract {
                    func bar() -> String {
                        return "hello"
                    }
                }
            }
        }
        "#;

        let handler = Handler::default();

        let mut lexer = Lexer::new(&source_code, handler);

        if let Ok(t) = lexer.lex() {
            for token in t.tokens() {
                println!("{:?} \n", token)
            }
        } else {
            println!(
                "error: {:?}, \nposition: line {:?}, col {:?}",
                lexer.errors().pop().expect("Error not found").error_kind(),
                lexer.errors().pop().expect("Error not found").line_col().0,
                lexer.errors().pop().expect("Error not found").line_col().1
            );
        }
    }
}
