use feo_ast::{
    attribute::OuterAttr,
    item::{
        EnumDef, EnumVariant, EnumVariantStruct, EnumVariantTuple, EnumVariantType, EnumVariants,
        VisibilityKind,
    },
    token::Token,
};
use feo_error::{error::CompilerError, parser_error::ParserErrorKind};
use feo_types::{
    delimiter::{DelimKind, DelimOrientation},
    keyword::KeywordKind,
    Delimiter, Identifier, Keyword,
};

use crate::{parse::ParseTerm, parser::Parser};

impl ParseTerm for EnumVariant {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(a) = OuterAttr::parse(parser)? {
            attributes.push(a);
            parser.next_token();
        }

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        if let Some(variant_name) = parser.peek_current::<Identifier>() {
            parser.next_token();

            let variant_type_opt = if let Some(v) = EnumVariantType::parse(parser)? {
                parser.next_token();
                Some(v)
            } else {
                None
            };

            match attributes.is_empty() {
                true => Ok(Some(EnumVariant {
                    attributes: Some(attributes),
                    visibility_opt,
                    variant_name,
                    variant_type_opt,
                })),

                false => Ok(Some(EnumVariant {
                    attributes: None,
                    visibility_opt,
                    variant_name,
                    variant_type_opt,
                })),
            }
        } else {
            Ok(None)
        }
    }
}

impl ParseTerm for EnumVariants {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for EnumVariantType {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for EnumVariantStruct {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for EnumVariantTuple {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        todo!()
    }
}

impl ParseTerm for EnumDef {
    fn parse(parser: &mut Parser) -> Result<Option<Self>, Vec<CompilerError>>
    where
        Self: Sized,
    {
        let mut attributes: Vec<OuterAttr> = Vec::new();

        while let Some(attr) = OuterAttr::parse(parser)? {
            attributes.push(attr);
            parser.next_token();
        }

        let visibility_opt = if let Some(v) = VisibilityKind::parse(parser)? {
            parser.next_token();
            Some(v)
        } else {
            None
        };

        let kw_enum_opt = parser.peek_current::<Keyword>();

        if let Some(Keyword {
            keyword_kind: KeywordKind::KwEnum,
            ..
        }) = kw_enum_opt
        {
            parser.next_token();

            if let Some(enum_name) = parser.peek_current::<Identifier>() {
                parser.next_token();

                let open_brace_opt = parser.peek_current::<Delimiter>();

                if let Some(Delimiter {
                    delim: (DelimKind::Brace, DelimOrientation::Open),
                    ..
                }) = open_brace_opt
                {
                    parser.next_token();

                    let enum_variants_opt = if let Some(e) = EnumVariants::parse(parser)? {
                        parser.next_token();
                        Some(e)
                    } else {
                        None
                    };

                    let close_brace_opt = parser.peek_current::<Delimiter>();

                    if let Some(Delimiter {
                        delim: (DelimKind::Brace, DelimOrientation::Close),
                        ..
                    }) = close_brace_opt
                    {
                        match attributes.is_empty() {
                            true => {
                                return Ok(Some(EnumDef {
                                    attributes: None,
                                    visibility_opt,
                                    kw_enum: kw_enum_opt.unwrap(),
                                    enum_name,
                                    open_brace: open_brace_opt.unwrap(),
                                    enum_variants_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }

                            false => {
                                return Ok(Some(EnumDef {
                                    attributes: Some(attributes),
                                    visibility_opt,
                                    kw_enum: kw_enum_opt.unwrap(),
                                    enum_name,
                                    open_brace: open_brace_opt.unwrap(),
                                    enum_variants_opt,
                                    close_brace: close_brace_opt.unwrap(),
                                }))
                            }
                        }
                    }

                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`}`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                } else {
                    parser.log_error(ParserErrorKind::UnexpectedToken {
                        expected: "`{`".to_string(),
                        found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                    });
                }
            } else {
                parser.log_error(ParserErrorKind::UnexpectedToken {
                    expected: "identifier".to_string(),
                    found: parser.current_token().unwrap_or(Token::EOF).to_string(),
                });
            }
        } else {
            return Ok(None);
        }

        Err(parser.errors())
    }
}
