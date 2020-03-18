use crate::Item;
use quote::format_ident;
use syn::parse_quote;
use syn::{parse::Parse, Block, ImplItem, ReturnType, Signature, TraitItem};

fn underline2camel(ident: &String) -> String {
    let v = ident.split('_');
    let mut r = String::from("FutureReturnType");
    for seg in v {
        r.push_str(&seg[0..1].to_uppercase());
        r.push_str(&seg[1..seg.len()]);
    }
    r
}

fn process_signature<T: Parse>(sig: &mut Signature, is_impl: bool) -> T {
    let origin_name = sig.ident.to_string();
    let camel_name = underline2camel(&origin_name);
    // modify name.
    let associated_type_name = format_ident!("ReturnType{}", camel_name);
    let associated_type = match &sig.output {
        ReturnType::Default => {
            if is_impl {
                parse_quote! {
                    type #associated_type_name<'a> = impl core::future::Future<Output = ()>;
                }
            } else {
                parse_quote! {
                    type #associated_type_name<'a>: core::future::Future<Output = ()>;
                }
            }
        }

        ReturnType::Type(_, t) => {
            if is_impl {
                parse_quote! {
                    type #associated_type_name<'a>  = impl core::future::Future<Output = #t>;
                }
            } else {
                parse_quote! {
                    type #associated_type_name<'a>: core::future::Future<Output = #t>;
                }
            }
        }
    };
    sig.asyncness = None;
    sig.output = parse_quote! {
        -> Self::#associated_type_name<'_>
    };
    associated_type
}

fn process_body_impl(block: &mut Block) {
    *block = parse_quote! {
        {
            async move #block
        }
    }
}

fn process_trait_default(block: &mut Block) {
    *block = parse_quote! {
        {
            compile_error!("Can't support trait method default implementation currently.");
            async move #block
        }
    }
}

pub fn expand(input: &mut Item) {
    match input {
        Item::Trait(input) => {
            let mut associated_types = Vec::new();
            for inner in &mut input.items {
                if let TraitItem::Method(method) = inner {
                    let sig = &mut method.sig;
                    if sig.asyncness.is_some() {
                        // process signature
                        let associated = process_signature::<TraitItem>(sig, false);
                        associated_types.push(associated);
                        if let Some(block) = &mut method.default {
                            // is default implementation.
                            process_trait_default(block);
                        }
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
                    if sig.asyncness.is_some() {
                        let associated = process_signature::<ImplItem>(sig, true);
                        associated_types.push(associated);
                        process_body_impl(&mut method.block);
                    }
                }
            }
            input.items.append(&mut associated_types);
        }
    }
}
