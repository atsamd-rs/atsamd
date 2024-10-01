//! This crate contains proc-macros to be used by the `atsamd-hal` crate. It is
//! not intended to be used outside this crate, and no stability guarantees are
//! made.
//!
//! The main purpose of this crate is to separate the task of writing the code
//! to support peripherals of the atsamd families from the task of figuring out
//! which specific devices has those peripherals.
//!
//! The actual mapping of devices to peripherals is specified in the
//! `devices.yaml` file. In the `atsamd-hal` crate you then only need to care
//! about the peripherals themselves (and their different variants).
//!
//! To use the macros in this crate, you need to specify a **peripheral
//! expression**, which can be one of the following:
//!
//! - A peripheral from `devices.yaml` in the form of a string. Examples:
//!   `"serial-numbers"` or `"sercom3"`.
//! - A peripheral from `devices.yaml` suffixed with the device family.
//!   Examples: `"serial-numbers-d11"` or `"sercom3-d5x"`
//! - A pin from `devices.yaml`. Examples: `"pb22"`.
//! - An expression of the form `any([peripheral expression], ...)`. Example:
//!   `any("pm-d11", "pm-d21", "rstc-d5x")`.
//! - An expression of the form `all([peripheral expression], ...)`. Example:
//!   `all("tc4", "tc5")`.

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

mod error;
mod generation;
mod parsing;

use error::Error;
use generation::{add_cfgs_to_input, cfg_args, gen_cfgs, hal_expr_to_devices};
use parsing::{eat_attribute, eat_eof, eat_group, eat_hal_expr, eat_operator, eat_string_literal};

/// Attribute macro which expands to a suitable `#[cfg(...)]` expression.
///
/// It can be used like `#[hal_cfg([peripheral expression])]`.
///
/// The macro will look up all devices that fulfill the expression and expand
/// the macro into a cfg attribute of the form `#[cfg(any(feature = "device1",
/// feature = "device2", ...)]`.
#[proc_macro_attribute]
pub fn hal_cfg(args: TokenStream, input: TokenStream) -> TokenStream {
    hal_cfg_impl(args).map_or_else(
        |e| e.to_compile_error("hal_cfg"),
        |cfgs| add_cfgs_to_input(cfgs, input),
    )
}

fn hal_cfg_impl(args: TokenStream) -> Result<Group, Error> {
    let mut args = args.into_iter().peekable();
    let expr = eat_hal_expr(&mut args)?;
    if args.peek().is_some() {
        eat_operator(",", &mut args)?;
    }
    eat_eof(&mut args)?;
    let cfgs = gen_cfgs(&expr)?;
    Ok(cfgs)
}

/// Macro which expands to a `mod foo;` item with different paths for each
/// device.
///
/// It can be used like this:
///
/// ```ignore
/// #[hal_module(
///     any("nvmctrl-d11", "nvmctrl-d21") => "calibration/d11.rs",
///     "nvmctrl-d5x" => "calibration/d5x.rs",
/// )]
/// pub mod calibration {}
///
/// #[hal_module("aes")
/// pub mod aes {}
/// ```
///
/// This will then expand to something of the form:
/// ```ignore
/// #[cfg(any(feature = "samd11c", ...))]
/// #[path = "calibration/d11.rs"]
/// pub mod calibration;
///
/// #[cfg(any(feature = "samd51g", ...))]
/// #[path = "calibration/d5x.rs"]
/// pub mod calibration;
///
/// #[cfg(any(feature = "samd51g", ...))]
/// pub mod aes;
/// ```
///
/// Ideally you would be to write `pub mod calibration;` instead of
/// `pub mod calibration {}`, but unfortunately non-inline modules are not
/// currently supposed in proc macros. See
/// [rust#54727](https://github.com/rust-lang/rust/issues/54727) for details.
#[proc_macro_attribute]
pub fn hal_module(args: TokenStream, input: TokenStream) -> TokenStream {
    hal_module_impl(args, input).unwrap_or_else(|e| e.to_compile_error("hal_module"))
}

fn hal_module_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let mut args = args.into_iter().peekable();
    let args = &mut args;

    let input = input
        .into_iter()
        .map(|token| {
            // Replace `{}` with `;`
            if let TokenTree::Group(g) = &token {
                if g.delimiter() == Delimiter::Brace && g.stream().into_iter().count() == 0 {
                    return TokenTree::Punct(Punct::new(';', Spacing::Alone));
                }
            }
            token
        })
        .collect::<Vec<_>>();

    let mut out = TokenStream::new();

    while args.peek().is_some() {
        let hal_expr = eat_hal_expr(args)?;

        out.extend([
            TokenTree::Punct(Punct::new('#', Spacing::Alone)),
            TokenTree::Group(gen_cfgs(&hal_expr)?),
        ]);

        if matches!(args.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '=') {
            eat_operator("=>", args)?;
            let path = eat_string_literal(args)?;
            out.extend([
                TokenTree::Punct(Punct::new('#', Spacing::Alone)),
                TokenTree::Group(Group::new(
                    Delimiter::Bracket,
                    TokenStream::from_iter([
                        TokenTree::Ident(Ident::new("path", Span::call_site())),
                        TokenTree::Punct(Punct::new('=', Spacing::Alone)),
                        TokenTree::Literal(Literal::string(&path)),
                    ]),
                )),
            ]);
        }

        out.extend(input.iter().cloned());

        if args.peek().is_some() {
            eat_operator(",", args)?;
        }
    }
    eat_eof(args)?;

    Ok(out)
}

/// Helper macro to allow using `#[hal_cfg(..)]` macro in more places
///
/// Normally the `#[cfg(..)]` macro is allowed in many more places than
/// proc-macros, such as directly on a statement. This mitigates that
/// restriction.
///
/// It can be used like this:
///
/// ```ignore
/// #[hal_macro_helper]
/// struct MyStruct {
///     #[hal_cfg("sercom0")]
///     my_field: String
/// }
/// ```
///
/// This works, because attributes are allowed on the outer item.
///
/// The `#[hal_macro_helper]` will search through the item and replace all
/// instances of the `#[hal_cfg(..)]` attribute with the corresponding
/// `#[cfg(..)]` attribute instead. This way the inner attributes are
/// technically not interpreted as a proc-macro at all.
#[proc_macro_attribute]
pub fn hal_macro_helper(_args: TokenStream, input: TokenStream) -> TokenStream {
    hal_macro_helper_impl(input).unwrap_or_else(|e| e.to_compile_error("hal_macro_helper"))
}

fn hal_macro_helper_impl(input: TokenStream) -> Result<TokenStream, Error> {
    let input = input.into_iter();
    let mut out = Vec::with_capacity(input.size_hint().0);
    let mut last_was_pound = false;

    for mut arg in input {
        let saved_last_was_pound = std::mem::replace(&mut last_was_pound, false);
        match &mut arg {
            TokenTree::Group(group) => {
                if saved_last_was_pound {
                    replace_inner_macros(group)?;
                } else {
                    let mut new_group =
                        Group::new(group.delimiter(), hal_macro_helper_impl(group.stream())?);
                    new_group.set_span(group.span());
                    *group = new_group;
                }
            }
            TokenTree::Punct(p) if p.as_char() == '#' && p.spacing() == Spacing::Alone => {
                last_was_pound = true;
            }
            _ => (),
        }
        out.push(arg);
    }
    Ok(out.into_iter().collect())
}

fn replace_inner_macros(group: &mut Group) -> Result<(), Error> {
    let mut tokens = group.stream().into_iter();
    let Some(TokenTree::Ident(func)) = tokens.next() else {
        return Ok(());
    };

    if func.to_string().as_str() != "hal_cfg" {
        return Ok(());
    }

    let Some(TokenTree::Group(inner_group)) = tokens.next() else {
        return Ok(());
    };

    if inner_group.delimiter() != Delimiter::Parenthesis {
        return Ok(());
    }

    if tokens.next().is_some() {
        return Ok(());
    }

    let mut new_group = hal_cfg_impl(inner_group.stream())?;
    new_group.set_span(group.span());
    *group = new_group;

    Ok(())
}

/// Helper macro to make conditional docs work nicer
///
/// Can be used like this:
///
/// ```ignore
/// #[hal_docs(
///     {
///         /// Example struct!
///         ///
///         /// It can do things!
///     }
///     "usb" => {
///         ///
///         /// It also supports usb on this device.
///     }
/// )]
/// pub struct ExampleStruct;
/// ```
#[proc_macro_attribute]
pub fn hal_docs(args: TokenStream, input: TokenStream) -> TokenStream {
    hal_docs_impl(args, input).unwrap_or_else(|e| e.to_compile_error("hal_docs"))
}

fn hal_docs_impl(args: TokenStream, input: TokenStream) -> Result<TokenStream, Error> {
    let mut args = args.into_iter().peekable();
    let args = &mut args;

    let mut out = TokenStream::new();

    while args.peek().is_some() {
        let mut attribute_formatter = None;

        if !matches!(args.peek(), Some(TokenTree::Group(_))) {
            let expr = eat_hal_expr(args)?;
            eat_operator("=>", args)?;

            let mut cfg_args = cfg_args(hal_expr_to_devices(&expr)?);
            cfg_args.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
            attribute_formatter = Some(move |attribute_body: TokenStream| {
                TokenStream::from_iter([
                    TokenTree::Ident(Ident::new("cfg_attr", Span::call_site())),
                    TokenTree::Group(Group::new(
                        Delimiter::Parenthesis,
                        TokenStream::from_iter(cfg_args.iter().cloned().chain(attribute_body)),
                    )),
                ])
            });
        }

        let group = eat_group(Delimiter::Brace, args)?;
        let mut items = group.stream().into_iter().peekable();

        while items.peek().is_some() {
            let mut attribute = eat_attribute(&mut items)?;
            if let Some(attribute_formatter) = &mut attribute_formatter {
                attribute.inner_stream = attribute_formatter(attribute.inner_stream);
            }
            out.extend([
                attribute.pound,
                TokenTree::Group(Group::new(Delimiter::Bracket, attribute.inner_stream)),
            ])
        }
    }
    eat_eof(args)?;

    out.extend(input);

    Ok(out)
}
