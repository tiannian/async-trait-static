use crate::Item;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse_quote;
use syn::{
    Block, Ident, ImplItem, ImplItemType, ItemType, ReturnType, Signature, TraitItem, TraitItemType,
};

fn get_async_block(_sig: &Signature, block: &Block) -> Block {
    let t: Block = parse_quote! {
        {
            let f = async move || #block;
            f()
        }
    };
    t
}

fn underline2camel(ident: &String) -> String {
    let v = ident.split('_');
    let mut r = String::from("FutureReturnType");
    for seg in v {
        r.push_str(&seg[0..1].to_uppercase());
        r.push_str(&seg[1..seg.len()]);
    }
    r
}

fn generate_signature_for_trait(sig: &mut Signature) -> TraitItemType {
    // generate associated type
    let name = &sig.ident.to_string();
    let camel_name = Ident::new(&underline2camel(&name.to_string()), Span::call_site());
    let true_type = match &sig.output {
        ReturnType::Default => {
            let tit: TraitItemType = parse_quote! {
                type #camel_name: core::future::Future<Output = ()>;
            };
            tit
        }
        ReturnType::Type(_, t) => {
            let tit: TraitItemType = parse_quote! {
                type #camel_name: core::future::Future<Output = #t>;
            };
            tit
        }
    };
    let new_output: ReturnType = parse_quote!(-> Self::#camel_name);
    sig.output = new_output;
    true_type
}

fn generate_signature_for_impl(sig: &mut Signature) -> ImplItemType {
    // generate associated type
    let name = &sig.ident.to_string();
    let camel_name = Ident::new(&underline2camel(&name.to_string()), Span::call_site());
    let true_type = match &sig.output {
        ReturnType::Default => {
            let tit: ImplItemType = parse_quote! {
                type #camel_name = impl core::future::Future<Output = ()>;
            };
            tit
        }
        ReturnType::Type(_, t) => {
            let tit: ImplItemType = parse_quote! {
                type #camel_name = impl core::future::Future<Output = #t>;
            };
            tit
        }
    };
    let new_output: ReturnType = parse_quote!(-> Self::#camel_name);
    sig.output = new_output;
    true_type
}

fn generate_signature_for_default(trait_prefix: &String, sig: &mut Signature) -> ItemType {
    // generate associated type
    let name = sig.ident.to_string() + trait_prefix;
    let camel_name = Ident::new(&underline2camel(&name.to_string()), Span::call_site());
    let true_type = match &sig.output {
        ReturnType::Default => {
            let tit: ItemType = parse_quote! {
                type #camel_name = impl core::future::Future<Output = ()>;
            };
            tit
        }
        ReturnType::Type(_, t) => {
            let tit: ItemType = parse_quote! {
                type #camel_name = impl core::future::Future<Output = #t>;
            };
            tit
        }
    };
    let new_output: ReturnType = parse_quote!(-> #camel_name);
    sig.output = new_output;
    true_type
}
pub fn expand(input: &mut Item) -> TokenStream {
    let mut type_alias_sum = Vec::new();
    match input {
        Item::Trait(input) => {
            let mut associated_types = Vec::new();
            for inner in &mut input.items {
                if let TraitItem::Method(method) = inner {
                    let sig = &mut method.sig;
                    if sig.asyncness.is_some() && method.default.is_none() {
                        let associated_type = generate_signature_for_trait(sig);
                        sig.asyncness = None;
                        associated_types.push(TraitItem::Type(associated_type));
                        // TODO: Deal body?
                    }
                    if sig.asyncness.is_some() && method.default.is_some() {
                        let block = &method.default;
                        method.default = Some(get_async_block(sig, block.as_ref().unwrap()));
                        let type_alias =
                            generate_signature_for_default(&input.ident.to_string(), sig);
                        type_alias_sum.push(type_alias);
                        sig.asyncness = None;
                    }
                }
            }
            input.items.append(&mut associated_types);
        }
        Item::Impl(input) => {
            let mut associated_types = Vec::new();
            for inner in &mut input.items {
                if let ImplItem::Method(method) = inner {
                    let sig = &mut method.sig;
                    method.block = get_async_block(sig, &method.block);
                    let associated_type = generate_signature_for_impl(sig);
                    sig.asyncness = None;
                    associated_types.push(ImplItem::Type(associated_type));
                    // convert body.
                }
            }
            input.items.append(&mut associated_types);
        }
    }
    quote! {
        #input

        #( #type_alias_sum )*
    }
}
