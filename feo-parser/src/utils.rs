use feo_ast::{
    expression::{TermCollection, Value, ValueCollection},
    item::{PathCollection, VisibilityKind},
    statement::Statement,
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    punctuation::PuncKind,
    Delimiter, Punctuation,
};

use crate::{
    parse::{ParseItem, ParseStatement, ParseTerm},
    parser::Parser,
    test_utils::{self, LogMsgType},
};

/// Helper function that collects attributes during parsing
/// `T` should only be an `InnerAttr` or `OuterAttr`
// TODO: restrict `T` to be either an `InnerAttr` or `OuterAttr` (using traits)
pub fn get_attributes<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    // test_utils::log_msg(LogMsgType::Enter, "`get_attributes()`", parser);

    let mut attributes: Vec<T> = Vec::new();

    while let Some(a) = T::parse(parser)? {
        attributes.push(a);
        parser.next_token();
    }

    // TODO: remove after initial debugging
    println!("number of attributes: {}", attributes.len());

    // test_utils::log_msg(LogMsgType::Exit, "`get_attributes()`", parser);

    if attributes.is_empty() {
        Ok(None)
    } else {
        Ok(Some(attributes))
    }
}

/// Helper function that collects `Item` during parsing.
/// e.g., `ConstVarDef`, `FuncDef`, `StructDef`, etc.
pub fn get_items<T: ParseItem>(parser: &mut Parser) -> Result<Option<Vec<T>>, Vec<CompilerError>> {
    test_utils::log_msg(LogMsgType::Enter, "`get_items()`", parser);

    let mut items: Vec<T> = Vec::new();

    while let Some(i) = T::parse(parser)? {
        parser.next_token();
        items.push(i);
    }

    // TODO: remove after initial debugging
    println!("number of items: {}", items.len());

    test_utils::log_msg(LogMsgType::Exit, "`get_items()`", parser);

    if items.is_empty() {
        Ok(None)
    } else {
        Ok(Some(items))
    }
}

/// Helper function that collects path elements during parsing (usually `Identifier` or `Self`)
/// `T` should resolve to some `PathIdenSegmentKind` or `SimplePathSegmentKind`
pub fn get_path_collection<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<PathCollection<T>>, Vec<CompilerError>> {
    test_utils::log_msg(LogMsgType::Enter, "`get_path_collection()`", parser);

    if let Some(root_path) = T::parse(parser)? {
        if let Some(Punctuation {
            punc_kind: PuncKind::DblColon,
            ..
        }) = parser.peek_current()
        {
            parser.next_token();

            if let Some(Delimiter {
                delim: (DelimKind::Brace, DelimOrientation::Open),
                ..
            }) = parser.peek_current()
            {
                parser.next_token();

                let path_suffixes = if let Some(inner_paths) = get_term_collection::<T>(parser)? {
                    Some(Box::new(inner_paths))
                } else {
                    None
                };

                if let Some(Punctuation {
                    punc_kind: PuncKind::ColonColonAsterisk,
                    ..
                }) = parser.peek_current()
                {
                    parser.next_token();
                    skip_trailing_comma(parser)?;
                }

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Close),
                    ..
                }) = parser.peek_current()
                {
                    test_utils::log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

                    return Ok(Some(PathCollection {
                        root_path: Box::new(root_path),
                        path_suffixes,
                    }));
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "`{`".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            test_utils::log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

            return Ok(Some(PathCollection {
                root_path: Box::new(root_path),
                path_suffixes: None,
            }));
        }
    } else {
        test_utils::log_msg(LogMsgType::Exit, "`get_path_collection()`", parser);

        return Ok(None);
    }

    Err(parser.errors())
}

/// Helper function that collects `Statement` during parsing.
/// Statements include all `Item`, as well as `ExprStatement` and `LetStatement`
pub fn get_statements(parser: &mut Parser) -> Result<Option<Vec<Statement>>, Vec<CompilerError>> {
    test_utils::log_msg(LogMsgType::Enter, "`get_statements()`", parser);

    let mut statements: Vec<Statement> = Vec::new();

    while let Some(s) = Statement::parse(parser)? {
        statements.push(s);

        if let Some(Delimiter {
            delim: (DelimKind::Brace, DelimOrientation::Close),
            ..
        }) = parser.peek_current()
        {
            break;
        }
    }

    // TODO: remove after initial debugging
    println!("number of statements: {}", statements.len());

    test_utils::log_msg(LogMsgType::Exit, "`get_statements()`", parser);

    if statements.is_empty() {
        return Ok(None);
    } else {
        return Ok(Some(statements));
    }
}

/// Helper function that collects terms during parsing (i.e., elements of `Item` and `Expression`)
pub fn get_term_collection<T: ParseTerm>(
    parser: &mut Parser,
) -> Result<Option<TermCollection<T>>, Vec<CompilerError>> {
    // test_utils::log_msg(LogMsgType::Enter, "`get_term_collection()`", parser);

    let mut terms: Vec<T> = Vec::new();

    if let Some(first_term) = T::parse(parser)? {
        parser.next_token();

        while let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current()
        {
            parser.next_token();

            if let Some(next_term) = T::parse(parser)? {
                terms.push(next_term);
                parser.next_token();
            } else {
                break;
            }
        }

        // TODO: remove after initial debugging
        println!("number of terms: {}", terms.len() + 1);

        let subsequent_terms_opt = if terms.is_empty() { None } else { Some(terms) };

        skip_trailing_comma(parser)?;

        // test_utils::log_msg(LogMsgType::Exit, "`get_term_collection()`", parser);

        return Ok(Some(TermCollection::new(first_term, subsequent_terms_opt)));
    } else {
        return Ok(None);
    }
}

/// Helper function that collects `Value` (`Expression`) during parsing
pub fn get_value_collection(
    parser: &mut Parser,
) -> Result<Option<ValueCollection>, Vec<CompilerError>> {
    let mut values: Vec<Value> = Vec::new();

    if let Some(first_value) = Value::parse(parser)? {
        parser.next_token();

        while let Some(Punctuation {
            punc_kind: PuncKind::Comma,
            ..
        }) = parser.peek_current::<Punctuation>()
        {
            parser.next_token();

            if let Some(next_value) = Value::parse(parser)? {
                values.push(next_value);
                parser.next_token();
            } else {
                break;
            }
        }

        // TODO: remove after initial debugging
        println!("number of values: {}", values.len() + 1);

        test_utils::log_msg(LogMsgType::Exit, "`get_value_collection()`", parser);

        let subsequent_values_opt = if values.is_empty() {
            None
        } else {
            Some(values)
        };

        return Ok(Some(ValueCollection {
            first_value: Box::new(first_value),
            subsequent_values_opt,
        }));
    } else {
        return Ok(None);
    }
}

/// Helper function that returns `VisibilityKind` during parsing
pub fn get_visibility(parser: &mut Parser) -> Result<Option<VisibilityKind>, Vec<CompilerError>> {
    // test_utils::log_msg(LogMsgType::Enter, "`get_visibility()`", parser);

    if let Some(v) = VisibilityKind::parse(parser)? {
        parser.next_token();

        // test_utils::log_msg(LogMsgType::Exit, "`get_visibility()`", parser);
        Ok(Some(v))
    } else {
        // test_utils::log_msg(LogMsgType::Exit, "`get_visibility()`", parser);
        Ok(None)
    }
}

/// Helper function that skips trailing commas
pub fn skip_trailing_comma(parser: &mut Parser) -> Result<(), Vec<CompilerError>> {
    if let Some(Punctuation {
        punc_kind: PuncKind::Comma,
        ..
    }) = parser.peek_current::<Punctuation>()
    {
        test_utils::log_msg(LogMsgType::Detect, "trailing comma", parser);

        parser.next_token();
        Ok(())
    } else {
        Ok(())
    }
}
