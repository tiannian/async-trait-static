mod async_trait;
mod parse;
mod ritit;
// mod selector;

use parse::Item;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn async_trait(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as Item);
    async_trait::expand(&mut item);
    let tk = quote! {
        #item
    };
    let impl_tk = TokenStream::from(tk);
    ritit(_arg, impl_tk)
}

#[proc_macro_attribute]
pub fn ritit(_arg: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as Item);
    ritit::expand(item)
}
