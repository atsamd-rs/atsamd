use std::collections::BTreeSet;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::{error::Error, parsing::HalExpr};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn all_devices() -> BTreeSet<&'static str> {
    ALL_DEVICES.iter().copied().collect()
}

fn lookup_peripheral(peripheral: &str) -> Result<&'static [&'static str], Error> {
    if let Some(cur_devices) = PERIPHERALS.get(peripheral) {
        Ok(*cur_devices)
    } else {
        Err(Error::UnknownPeripheral {
            peripheral: peripheral.to_string(),
        })
    }
}

fn cfg_token_tree<'a>(devices: impl IntoIterator<Item = &'a str>) -> Group {
    let cfg_args = cfg_args(devices);
    let cfg_args = TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        cfg_args.into_iter().collect(),
    ));

    Group::new(
        Delimiter::Bracket,
        [
            TokenTree::Ident(Ident::new("cfg", Span::call_site())),
            cfg_args,
        ]
        .into_iter()
        .collect(),
    )
}

pub fn cfg_args<'a>(devices: impl IntoIterator<Item = &'a str>) -> Vec<TokenTree> {
    let mut inner = Vec::new();
    for device in devices {
        if !inner.is_empty() {
            inner.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
        }
        inner.push(TokenTree::Ident(Ident::new("feature", Span::call_site())));
        inner.push(TokenTree::Punct(Punct::new('=', Spacing::Alone)));
        inner.push(TokenTree::Literal(Literal::string(device)));
    }
    let any_args = TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        inner.into_iter().collect(),
    ));
    vec![
        TokenTree::Ident(Ident::new("any", Span::call_site())),
        any_args,
    ]
}

pub fn hal_expr_to_devices(expr: &HalExpr) -> Result<BTreeSet<&'static str>, Error> {
    match expr {
        HalExpr::All(args) => {
            let mut devices = all_devices();
            for arg in args {
                devices = devices
                    .intersection(&hal_expr_to_devices(arg)?)
                    .copied()
                    .collect();
            }
            Ok(devices)
        }
        HalExpr::Any(args) => {
            let mut devices = BTreeSet::new();
            for arg in args {
                devices.extend(hal_expr_to_devices(arg)?);
            }
            Ok(devices)
        }
        HalExpr::Peripheral(lit) => Ok(lookup_peripheral(lit)?.iter().copied().collect()),
    }
}

pub fn gen_cfgs(expr: &HalExpr) -> Result<Group, Error> {
    Ok(cfg_token_tree(hal_expr_to_devices(expr)?))
}

pub fn add_cfgs_to_input(cfgs: Group, input: TokenStream) -> TokenStream {
    [
        TokenTree::Punct(Punct::new('#', Spacing::Alone)),
        TokenTree::Group(cfgs),
    ]
    .into_iter()
    .chain(input)
    .collect::<TokenStream>()
}
