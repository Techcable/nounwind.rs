//! The underlying implementation of the `#[nounwind]` attribute macro.

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, parse_quote};

struct Empty;
impl Parse for Empty {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        Ok(Empty)
    }
}

#[proc_macro_attribute]
pub fn nounwind(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let _: Empty = parse_macro_input!(attr as Empty);
    let input = parse_macro_input!(item as syn::ItemFn);
    do_nounwind(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn do_nounwind(mut item: syn::ItemFn) -> syn::Result<TokenStream> {
    let old_block = std::mem::replace(
        &mut item.block,
        Box::new(parse_quote!({ compile_error!("dummy value") })),
    );
    item.block = Box::new(parse_quote!({
        nounwind::abort_unwind(#[inline(always)] move || {
            #old_block
        })
    }));
    Ok(item.into_token_stream())
}
