extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{ExprLit, Lit};

#[proc_macro]
pub fn factorial(input: TokenStream) -> TokenStream {
    match syn::parse(input) {
        Ok(ExprLit {
            lit: Lit::Int(lit), ..
        }) => {
            let result: u64 = (1..=lit.value()).product();
            result.to_string().parse().unwrap()
        }
        _ => quote!(compile_error!("argument must be an integer literal")).into(),
    }
}
