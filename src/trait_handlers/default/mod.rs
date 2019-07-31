mod models;

mod default_struct;
mod default_enum;
mod default_union;

use super::TraitHandler;

use crate::Trait;
use crate::proc_macro2::TokenStream;
use crate::syn::{DeriveInput, Meta, Data};

use default_struct::DefaultStructHandler;
use default_enum::DefaultEnumHandler;
use default_union::DefaultUnionHandler;

pub struct DefaultHandler;

impl TraitHandler for DefaultHandler {
    fn trait_meta_handler(ast: &DeriveInput, tokens: &mut TokenStream, traits: &[Trait], meta: &Meta) {
        match ast.data {
            Data::Struct(_) => {
                DefaultStructHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Enum(_) => {
                DefaultEnumHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
            Data::Union(_) => {
                DefaultUnionHandler::trait_meta_handler(ast, tokens, traits, meta);
            }
        }
    }
}