#![doc = include_str!("../README.md")]

mod macro_impl;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rstest(args: TokenStream, item: TokenStream) -> TokenStream {
    // Implementation kept in macro_impl::test_attribute_impl for historical reasons.
    macro_impl::test_attribute_impl(args, item)
}
