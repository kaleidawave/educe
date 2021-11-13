#![doc = include_str!("../README.md")]

#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;

extern crate syn;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate enum_ordinalize;

mod panic;
mod support_traits;
mod trait_handlers;

use std::collections::BTreeMap;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Meta, NestedMeta};

use support_traits::Trait;
use trait_handlers::TraitHandler;

fn derive_input_handler(ast: DeriveInput) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut trait_meta_map: BTreeMap<Trait, Meta> = BTreeMap::new();

    for attr in ast.attrs.iter() {
        if let Some(attr_meta_name) = attr.path.get_ident() {
            if attr_meta_name == "educe" {
                let attr_meta = attr.parse_meta().unwrap();

                match attr_meta {
                    Meta::List(list) => {
                        for p in list.nested {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    let t = Trait::from_str(meta_name);

                                    if trait_meta_map.contains_key(&t) {
                                        panic::reuse_a_trait(t);
                                    }

                                    trait_meta_map.insert(t, meta);
                                }
                                NestedMeta::Lit(_) => {
                                    panic::educe_format_incorrect();
                                }
                            }
                        }
                    }
                    _ => {
                        panic::educe_format_incorrect();
                    }
                }
            }
        }
    }

    let traits: Vec<Trait> = trait_meta_map.keys().copied().collect();

    #[cfg(feature = "Debug")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Debug) {
            trait_handlers::debug::DebugHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "PartialEq")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::PartialEq) {
            trait_handlers::partial_eq::PartialEqHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "Eq")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Eq) {
            trait_handlers::eq::EqHandler::trait_meta_handler(&ast, &mut tokens, &traits, meta);
        }
    }

    #[cfg(feature = "PartialOrd")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::PartialOrd) {
            trait_handlers::partial_ord::PartialOrdHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "Ord")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Ord) {
            trait_handlers::ord::OrdHandler::trait_meta_handler(&ast, &mut tokens, &traits, meta);
        }
    }

    #[cfg(feature = "Hash")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Hash) {
            trait_handlers::hash::HashHandler::trait_meta_handler(&ast, &mut tokens, &traits, meta);
        }
    }

    #[cfg(feature = "Default")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Default) {
            trait_handlers::default::DefaultHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "Clone")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Clone) {
            trait_handlers::clone::CloneHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "Copy")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Copy) {
            trait_handlers::copy::CopyHandler::trait_meta_handler(&ast, &mut tokens, &traits, meta);
        }
    }

    #[cfg(feature = "Deref")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::Deref) {
            trait_handlers::deref::DerefHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    #[cfg(feature = "DerefMut")]
    {
        if let Some(meta) = trait_meta_map.get(&Trait::DerefMut) {
            trait_handlers::deref_mut::DerefMutHandler::trait_meta_handler(
                &ast,
                &mut tokens,
                &traits,
                meta,
            );
        }
    }

    if tokens.is_empty() {
        panic::derive_attribute_not_set_up_yet("Educe");
    }

    tokens
}

#[proc_macro_derive(Educe, attributes(educe))]
pub fn educe_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive_input_handler(syn::parse(input).unwrap()).into()
}
