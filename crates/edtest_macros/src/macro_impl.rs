use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

pub(crate) fn test_attribute_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = parse_macro_input!(input as ItemFn);

    let test_attr = if sig.asyncness.is_some() {
        quote!(
            #[test_log::test(::tokio::test)]
        )
    } else {
        quote!(#[test_log::test])
    };

    quote!(
        #[rstest::rstest]
        #(#attrs)*
        #test_attr
        #vis #sig #block
    )
    .into()
}
