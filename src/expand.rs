use crate::selector::Selector;
use crate::Item;
use syn::parse_quote;
use syn::{parse::Parse, Block, ImplItem, Signature, TraitItem};

fn process_signature<T: Parse>(sig: &mut Signature, is_impl: bool) -> T {
    let stor = Selector::new(sig);
    sig.asyncness = None;
    sig.output = stor.return_type();
    stor.associated_type(is_impl)
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
