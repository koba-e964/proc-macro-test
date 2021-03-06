extern crate proc_macro;
extern crate proc_macro2;
use quote::quote;

/// https://blog.rust-lang.org/2018/12/21/Procedural-Macros-in-Rust-2018.html
#[proc_macro_attribute]
pub fn hello(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as ::syn::ItemFn);
    let name = input.sig.ident;

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        fn #name() -> u32 { 42 }
    };
    result.into()
}

#[proc_macro]
pub fn make_pub(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let item: proc_macro2::TokenStream = item.into();
    let result = quote! {
        pub #item
    };
    result.into()
}
