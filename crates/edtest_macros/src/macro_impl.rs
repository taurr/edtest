use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn test_attribute_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let syn::ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = syn::parse_macro_input!(input as syn::ItemFn);

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
