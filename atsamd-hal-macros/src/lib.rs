use std::{borrow::Cow, collections::BTreeSet};

use proc_macro::{Ident, Spacing, TokenStream, TokenTree};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

fn gen_cfgs(peripherals: &str) -> Option<TokenStream> {
    let mut devices: BTreeSet<&'static str> = BTreeSet::new();
    for peripheral in peripherals.split_ascii_whitespace() {
        devices.extend(match_peripheral(&peripheral)?);
    }

    let mut s = "#[cfg(any(".to_string();

    for d in devices {
        s += format!("feature = {d:?}, ").as_str();
    }

    s += "))]\n";

    s.parse().ok()
}

#[proc_macro_attribute]
pub fn hal_cfgs(args: TokenStream, input: TokenStream) -> TokenStream {
    match hal_cfgs_impl(&mut args.into_iter(), input) {
        Some(stream) => stream,
        None => {
            let s = r#"compile_error!("hal_cfgs accepts a attribute takes exactly one argument, which must be a string");"#;
            s.parse().unwrap()
        }
    }
}

fn hal_cfgs_impl(
    args: &mut impl Iterator<Item = TokenTree>,
    input: TokenStream,
) -> Option<TokenStream> {
    let peripherals = eat_string_literal(args)?;
    eat_eof(args)?;

    let mut token_stream = gen_cfgs(&peripherals)?;
    token_stream.extend(input);
    Some(token_stream)
}

#[proc_macro]
pub fn hal_module_mapping(args: TokenStream) -> TokenStream {
    match hal_module_mapping_impl(args) {
        Some(stream) => stream,
        None => {
            let s = r#"compile_error!("hal_module_mapping was unable to parse its input");"#;
            s.parse().unwrap()
        }
    }
}

fn hal_module_mapping_impl(args: TokenStream) -> Option<TokenStream> {
    let mut args = args.into_iter().peekable();
    let args = &mut args;

    let mut is_pub = false;
    let mut module_name = eat_ident(args)?;
    if module_name.to_string() == "pub" {
        is_pub = true;
        module_name = eat_ident(args)?;
    }

    let mut out = TokenStream::new();

    while args.peek().is_some() {
        eat_operator(",", args)?;
        if args.peek().is_none() {
            break;
        }

        let peripherals = eat_string_literal(args)?;
        eat_operator("=>", args)?;
        let path = eat_string_literal(args)?;

        out.extend(gen_cfgs(&peripherals)?);
        out.extend(
            format!("#[path = {path:?}]")
                .parse::<TokenStream>()
                .unwrap(),
        );
        if is_pub {
            out.extend("pub mod".parse::<TokenStream>().unwrap());
        } else {
            out.extend("mod".parse::<TokenStream>().unwrap());
        }
        out.extend(std::iter::once(TokenTree::from(module_name.clone())));
        out.extend(";".parse::<TokenStream>().unwrap());
    }

    Some(out)
}

fn eat_operator(s: &str, args: &mut impl Iterator<Item = TokenTree>) -> Option<()> {
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        let TokenTree::Punct(punct) = args.next()? else {
            return None;
        };

        if punct.as_char() != c {
            return None;
        }

        match (chars.peek().is_some(), punct.spacing()) {
            (true, Spacing::Alone) => return None,
            (false, Spacing::Joint) => return None,
            _ => (),
        }
    }

    Some(())
}

fn eat_string_literal(args: &mut impl Iterator<Item = TokenTree>) -> Option<Cow<'static, str>> {
    Some(litrs::StringLit::try_from(args.next()?).ok()?.into_value())
}

fn eat_ident(args: &mut impl Iterator<Item = TokenTree>) -> Option<Ident> {
    if let TokenTree::Ident(ident) = args.next()? {
        Some(ident)
    } else {
        None
    }
}

fn eat_eof(args: &mut impl Iterator<Item = TokenTree>) -> Option<()> {
    if args.next().is_none() {
        Some(())
    } else {
        None
    }
}
