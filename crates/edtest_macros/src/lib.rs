#![doc = include_str!("../README.md")]

mod macro_impl;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn test(args: TokenStream, item: TokenStream) -> TokenStream {
    macro_impl::test_attribute_impl(args, item)
}
