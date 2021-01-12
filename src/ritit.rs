use crate::Item;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
// use syn::token::Add;
use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
    parse_quote, FnArg, GenericParam, Ident, ImplItem, ItemImpl, ItemTrait, ReturnType, Signature,
    Token, TraitItem, TraitItemType, Type, TypeParamBound,
};

fn get_return_bounds(return_type: &ReturnType) -> Option<Punctuated<TypeParamBound, Token![+]>> {
    if let ReturnType::Type(_rarrow, ty) = return_type {
        if let Type::ImplTrait(impl_trait) = &**ty {
            return Some(impl_trait.bounds.clone());
        }
    }
    None
}

fn get_generics(sig: &Signature) -> Punctuated<GenericParam, Token![,]> {
    let mut generics = Punctuated::<GenericParam, Token![,]>::new();

    for generic in &sig.generics.params {
        if let GenericParam::Type(tp) = generic {
            let ident = &tp.ident;
            let gp: GenericParam = parse_quote!(#ident);
            generics.push(gp);
        }

        if let GenericParam::Lifetime(_) = generic {
            generics.push(generic.clone());
        }
    }

    generics
}

fn modify_receiver(sig: &mut Signature) {
    let receiver = sig.receiver();
    if receiver.is_some() {
        let mut rr = receiver.unwrap().clone();
        if let FnArg::Receiver(r) = &mut rr {
            if r.reference.is_some() {
                r.reference.as_mut().unwrap().1 = parse_quote!('_async_lifetime);
                sig.inputs[0] = rr;
            }
        }
    }
}

fn process_trait(mut input: ItemTrait) -> TokenStream {
    let mut asses = Vec::new();
    let mut funcs = Vec::new();
    for item in &mut input.items {
        if let TraitItem::Method(method) = item {
            if let Some(bounds) = get_return_bounds(&method.sig.output) {
                let name = String::from("RititReturn")
                    + &method.sig.ident.to_string().to_case(Case::Pascal);
                let type_name = Ident::new(&name, Span::call_site());

                let generics = get_generics(&method.sig);

                modify_receiver(&mut method.sig);

                let mut func = method.clone();
                func.sig.output = parse_quote!(-> Self::#type_name<'_async_lifetime, #generics>);
                let async_lifetime: GenericParam = parse_quote!('_async_lifetime);
                func.sig.generics.params.insert(0, async_lifetime);
                funcs.push(TraitItem::Method(func));

                let associated_type: TraitItemType = parse_quote! {
                    type #type_name<'_async_lifetime, #generics>: #bounds;
                };

                if let Some(block) = &method.default {

                }

                asses.push(TraitItem::Type(associated_type));
            }
        }
    }

    asses.append(&mut funcs);

    input.items = asses;

    let tk = quote! {
        #input
    };
    TokenStream::from(tk)
}

fn process_impl(mut input: ItemImpl) -> TokenStream {
    let mut asses = Vec::new();
    let mut funcs = Vec::new();
    // let struct_name = input.
    for item in &mut input.items {
        if let ImplItem::Method(method) = item {
            if let Some(bounds) = get_return_bounds(&method.sig.output) {
                let name = String::from("RititReturn")
                    + &method.sig.ident.to_string().to_case(Case::Pascal);
                let type_name = Ident::new(&name, Span::call_site());

                let generics = get_generics(&method.sig);

                modify_receiver(&mut method.sig);

                let mut func = method.clone();
                func.sig.output = parse_quote!(-> Self::#type_name<'_async_lifetime, #generics>);
                let async_lifetime: GenericParam = parse_quote!('_async_lifetime);
                func.sig.generics.params.insert(0, async_lifetime);
                funcs.push(ImplItem::Method(func));

                let associated_type: ImplItem = parse_quote! {
                    type #type_name<'_async_lifetime, #generics> = impl #bounds;
                };
                asses.push(associated_type);
            }
        }
    }

    asses.append(&mut funcs);

    input.items = asses;

    let tk = quote! {
        #input
    };

    TokenStream::from(tk)
}

pub fn expand(input: Item) -> TokenStream {
    match input {
        Item::Trait(i) => process_trait(i),
        Item::Impl(i) => process_impl(i),
    }
}
