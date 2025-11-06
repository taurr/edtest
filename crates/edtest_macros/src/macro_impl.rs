use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use quote::quote;

fn expand_impl(_args: &TS2, input: &TS2) -> TS2 {
    let syn::ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = syn::parse2::<syn::ItemFn>(input.clone()).expect("parse function");

    let test_attr = if sig.asyncness.is_some() {
        quote!(
            #[test_log::test(::tokio::test)]
        )
    } else {
        quote!(#[test_log::test])
    };

    let name = &sig.ident;

    quote!(
        #[rstest::rstest]
        #(#attrs)*
        #test_attr
        #vis #sig {
            edtest::internal::on_test_enter(stringify!(#name));
            let _edtest_guard = edtest::TestGuard::new();
            #block
        }
    )
}

pub(crate) fn test_attribute_impl(args: TokenStream, input: TokenStream) -> TokenStream {
    let out = expand_impl(&TS2::from(args), &TS2::from(input));
    TokenStream::from(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::{format_ident, quote};

    fn expand(input: TS2) -> String {
        expand_impl(&TS2::new(), &input).to_string()
    }

    #[test]
    fn expands_sync_test_with_rstest_and_logging() {
        let name = format_ident!("sync_example");
        let input = quote! {
            fn #name(#[values(1,2)] x: u32) { let _ = x + 1; }
        };
        let out = expand(input);
        assert!(
            out.contains("rstest :: rstest"),
            "missing rstest attribute: {}",
            out
        );
        assert!(
            out.contains("test_log :: test"),
            "missing test-log attribute: {}",
            out
        );
        assert!(
            out.contains("edtest :: internal :: on_test_enter"),
            "missing enter hook: {}",
            out
        );
        assert!(
            out.contains("edtest :: TestGuard :: new"),
            "missing guard: {}",
            out
        );
    }

    #[test]
    fn expands_async_test_with_tokio_runtime() {
        let name = format_ident!("async_example");
        let input = quote! {
            async fn #name() { let _ = 1 + 1; }
        };
        let out = expand(input);
        assert!(
            out.contains("test_log :: test ("),
            "missing test-log attr: {}",
            out
        );
        assert!(
            out.contains(":: tokio :: test"),
            "missing tokio runtime: {}",
            out
        );
        assert!(
            out.contains("edtest :: internal :: on_test_enter"),
            "missing enter hook: {}",
            out
        );
        assert!(
            out.contains("edtest :: TestGuard :: new"),
            "missing guard: {}",
            out
        );
    }
}
