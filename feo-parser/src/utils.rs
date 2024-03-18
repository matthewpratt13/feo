use feo_ast::expression::{Value, ValueCollection};
use feo_error::error::CompilerError;
use feo_types::{punctuation::PuncKind, Punctuation};

use crate::{parse::ParseTerm, parser::Parser};

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

        skip_trailing_comma(parser)?;

        let subsequent_values_opt = if values.is_empty() {
            None
        } else {
            Some(values)
        };

        return Ok(Some(ValueCollection::new(
            first_value,
            subsequent_values_opt,
        )));
    } else {
        return Ok(None);
    }
}

pub fn skip_trailing_comma(parser: &mut Parser) -> Result<(), Vec<CompilerError>> {
    if let Some(Punctuation {
        punc_kind: PuncKind::Comma,
        ..
    }) = parser.peek_current::<Punctuation>()
    {
        parser.next_token();
        Ok(())
    } else {
        Ok(())
    }
}
