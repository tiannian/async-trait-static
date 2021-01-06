use crate::Item;
use syn::parse_quote;
use syn::{ImplItem, ImplItemMethod, ReturnType, TraitItem, TraitItemMethod};

fn process_signature_return(return_type: &mut ReturnType) {
    match return_type {
        ReturnType::Default => {
            *return_type = parse_quote! {
                -> impl core::future::Future<Output = ()>
            }
        }
        ReturnType::Type(_rarrow, ty) => {
            *return_type = parse_quote! {
                -> impl core::future::Future<Output = #ty>
            }
        }
    }
}

fn process_trait_method(item: &mut TraitItemMethod) {
    item.sig.asyncness = None;
    process_signature_return(&mut item.sig.output);
    if let Some(block) = &mut item.default {
        *block = parse_quote! {
            {
                async move #block
            }
        }
    }
}

fn process_impl_method(item: &mut ImplItemMethod) {
    item.sig.asyncness = None;
    process_signature_return(&mut item.sig.output);
    let block = item.block.clone();
    item.block = parse_quote! {
        {
            async move #block
        }
    }
}

pub fn expand(input: &mut Item) {
    match input {
        Item::Trait(input) => {
            for item in &mut input.items {
                match item {
                    TraitItem::Method(method) => process_trait_method(method),
                    _ => (),
                };
            }
        }
        Item::Impl(input) => {
            for item in &mut input.items {
                match item {
                    ImplItem::Method(method) => process_impl_method(method),
                    _ => (),
                }
            }
        }
    }
}
