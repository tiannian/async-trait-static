use quote::format_ident;
use syn::{parse::Parse, parse_quote, FnArg, Ident, ReturnType, Signature, Type};

#[derive(Debug)]
pub enum Selector {
    GATLifetime { name: Ident, t: Type },
    NoLifetime { name: Ident, t: Type },
    Unknown,
}

fn get_return_type(sig: &Signature) -> Type {
    match &sig.output {
        ReturnType::Default => parse_quote!(()),
        ReturnType::Type(_, t) => *(*t).clone(),
    }
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

fn get_camel_name_from_underline(ident: &Ident) -> Ident {
    let origin_name = ident.to_string();
    let camel_name = underline2camel(&origin_name);
    // modify name.
    format_ident!("ReturnType{}", camel_name)
}

fn gat_lifetime_associated_type<T: Parse>(name: &Ident, t: &Type, is_impl: bool) -> T {
    if is_impl {
        parse_quote! {
            type #name<'a> = impl core::future::Future<Output = #t>;
        }
    } else {
        parse_quote! {
            type #name<'a>: core::future::Future<Output = #t>;
        }
    }
}

fn no_lifetime_associated_type<T: Parse>(name: &Ident, t: &Type, is_impl: bool) -> T {
    if is_impl {
        parse_quote! {
            type #name = impl core::future::Future<Output = #t>;
        }
    } else {
        parse_quote! {
            type #name: core::future::Future<Output = #t>;
        }
    }
}

impl Selector {
    pub fn new(sig: &Signature) -> Selector {
        if let Some(a) = sig.receiver() {
            match a {
                FnArg::Receiver(r) => match r.reference {
                    Some(_) => Selector::GATLifetime {
                        name: get_camel_name_from_underline(&sig.ident),
                        t: get_return_type(sig),
                    },
                    None => Selector::NoLifetime {
                        name: get_camel_name_from_underline(&sig.ident),
                        t: get_return_type(sig),
                    },
                },
                _ => Selector::Unknown,
            }
        } else {
            Selector::NoLifetime {
                name: get_camel_name_from_underline(&sig.ident),
                t: get_return_type(sig),
            }
        }
    }

    pub fn associated_type<T: Parse>(&self, is_impl: bool) -> T {
        match self {
            Selector::GATLifetime { name, t } => gat_lifetime_associated_type(name, t, is_impl),
            Selector::NoLifetime { name, t } => no_lifetime_associated_type(name, t, is_impl),
            _ => parse_quote! { () },
        }
    }

    pub fn return_type(&self) -> ReturnType {
        match self {
            Selector::GATLifetime { name, t: _ } => parse_quote! { -> Self::#name<'a> },
            Selector::NoLifetime { name, t: _ } => parse_quote! { -> Self::#name },
            _ => parse_quote! { () },
        }
    }
}
