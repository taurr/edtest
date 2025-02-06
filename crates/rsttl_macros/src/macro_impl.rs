use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Signature};

pub(crate) fn test_attribute_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = parse_macro_input!(input as ItemFn);

    let test_attr = add_test_log(&sig);

    quote!(
        #[::rstest::rstest]
        #(#attrs)*
        #test_attr
        #vis #sig #block
    )
    .into()
}

#[cfg(all(not(feature = "test-log"), feature = "async-std"))]
fn add_test_log(sig: &Signature) -> proc_macro2::TokenStream {
    if sig.asyncness.is_some() {
        quote!(#[::async_std::test])
    } else {
        quote!()
    }
}

#[cfg(all(not(feature = "test-log"), not(feature = "async-std")))]
fn add_test_log(sig: &Signature) -> proc_macro2::TokenStream {
    if sig.asyncness.is_some() {
        quote!(#[::tokio::test])
    } else {
        quote!()
    }
}

#[cfg(all(feature = "test-log", feature = "async-std"))]
fn add_test_log(sig: &Signature) -> proc_macro2::TokenStream {
    if sig.asyncness.is_some() {
        quote!(#[::test_log::test(::async_std::test)])
    } else {
        quote!(#[::test_log::test])
    }
}

#[cfg(all(feature = "test-log", not(feature = "async-std")))]
fn add_test_log(sig: &Signature) -> proc_macro2::TokenStream {
    if sig.asyncness.is_some() {
        quote!(
            #[::test_log::test(::tokio::test)]
        )
    } else {
        quote!(#[::test_log::test])
    }
}
