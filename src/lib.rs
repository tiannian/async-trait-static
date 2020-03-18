mod expand;
mod parse;
// mod receiver;

use expand::expand;
use parse::Item;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_trait(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    expand(&mut item);
    let tk = quote!(#item);
    // println!("{}", tk);
    TokenStream::from(tk)
}
