use std::borrow::Cow;

use proc_macro::{Delimiter, Group, Spacing, TokenStream, TokenTree};

use crate::error::Error;

pub fn eat_operator(s: &str, args: &mut impl Iterator<Item = TokenTree>) -> Result<(), Error> {
    let mut chars = s.chars().peekable();

    macro_rules! bail {
        () => {
            return Err(Error::GenericParseError {
                expected: format!("operator {s:?}"),
            })
        };
    }

    while let Some(c) = chars.next() {
        let Some(TokenTree::Punct(punct)) = args.next() else {
            bail!();
        };

        if punct.as_char() != c {
            bail!()
        }

        match (chars.peek().is_some(), punct.spacing()) {
            (true, Spacing::Alone) => bail!(),
            (false, Spacing::Joint) => bail!(),
            _ => (),
        }
    }

    Ok(())
}

pub fn eat_string_literal(
    args: &mut impl Iterator<Item = TokenTree>,
) -> Result<Cow<'static, str>, Error> {
    let Some(lit) = args.next() else {
        return Err(Error::GenericParseError {
            expected: "string literal".to_string(),
        });
    };

    Ok(litrs::StringLit::try_from(lit)
        .map_err(|_| Error::GenericParseError {
            expected: "string literal".to_string(),
        })?
        .into_value())
}

pub fn eat_group(
    delimiter: Delimiter,
    args: &mut impl Iterator<Item = TokenTree>,
) -> Result<Group, Error> {
    let err = || Error::GenericParseError {
        expected: format!("group with delimiter {delimiter:?}"),
    };

    let Some(TokenTree::Group(group)) = args.next() else {
        return Err(err());
    };

    (group.delimiter() == delimiter)
        .then_some(group)
        .ok_or_else(err)
}

pub struct Attribute {
    pub pound: TokenTree,
    pub inner_stream: TokenStream,
}

pub fn eat_attribute(args: &mut impl Iterator<Item = TokenTree>) -> Result<Attribute, Error> {
    macro_rules! bail {
        () => {
            return Err(Error::GenericParseError {
                expected: "attribute".to_string(),
            })
        };
    }

    let Some(TokenTree::Punct(punct)) = args.next() else {
        bail!();
    };

    if punct.as_char() != '#' {
        bail!();
    }

    let group = eat_group(Delimiter::Bracket, args)?;

    Ok(Attribute {
        pound: TokenTree::Punct(punct),
        inner_stream: group.stream(),
    })
}

pub fn eat_eof(args: &mut impl Iterator<Item = TokenTree>) -> Result<(), Error> {
    if args.next().is_none() {
        Ok(())
    } else {
        Err(Error::GenericParseError {
            expected: "EOF".to_string(),
        })
    }
}

pub enum HalExpr {
    All(Vec<HalExpr>),
    Any(Vec<HalExpr>),
    Peripheral(std::borrow::Cow<'static, str>),
}

pub fn eat_hal_expr(expr: &mut impl Iterator<Item = TokenTree>) -> Result<HalExpr, Error> {
    match expr.next() {
        Some(TokenTree::Ident(ident)) => {
            let variant: Option<fn(_) -> _> = match ident.to_string().as_str() {
                "all" => Some(HalExpr::All),
                "any" => Some(HalExpr::Any),
                _ => None,
            };
            if let Some(variant) = variant {
                if let Some(TokenTree::Group(group)) = expr.next() {
                    if group.delimiter() == Delimiter::Parenthesis {
                        let mut inner = Vec::new();
                        let mut args = group.stream().into_iter().peekable();
                        while args.peek().is_some() {
                            inner.push(eat_hal_expr(&mut args)?);
                            if args.peek().is_none() {
                                break;
                            }
                            eat_operator(",", &mut args)?;
                        }
                        eat_eof(&mut args)?;
                        return Ok(variant(inner));
                    }
                }
            }
        }
        Some(TokenTree::Literal(lit)) => {
            if let Ok(string_lit) = litrs::StringLit::try_from(lit) {
                return Ok(HalExpr::Peripheral(string_lit.into_value()));
            }
        }
        _ => (),
    }
    Err(Error::GenericParseError {
        expected: "a string literal or an any()/all() expression".to_string(),
    })
}
