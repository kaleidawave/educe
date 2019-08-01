use super::super::super::{create_where_predicates_from_lit_str, create_where_predicates_from_generic_parameters};

use crate::Trait;
use crate::syn::{Meta, NestedMeta, Lit, Attribute, WherePredicate, GenericParam, punctuated::Punctuated, token::Comma};
use crate::panic;

#[derive(Clone)]
pub enum TypeAttributeBound {
    None,
    Auto,
    Custom(Punctuated<WherePredicate, Comma>),
}

impl TypeAttributeBound {
    pub fn into_punctuated_where_predicates_by_generic_parameters(self, params: &Punctuated<GenericParam, Comma>) -> Punctuated<WherePredicate, Comma> {
        match self {
            TypeAttributeBound::None => Punctuated::new(),
            TypeAttributeBound::Auto => create_where_predicates_from_generic_parameters(params, &syn::parse(quote!(core::clone::Clone).into()).unwrap()),
            TypeAttributeBound::Custom(where_predicates) => where_predicates
        }
    }

    pub fn into_punctuated_where_predicates_by_generic_parameters_with_copy(self, params: &Punctuated<GenericParam, Comma>) -> Punctuated<WherePredicate, Comma> {
        match self {
            TypeAttributeBound::None => Punctuated::new(),
            TypeAttributeBound::Auto => create_where_predicates_from_generic_parameters(params, &syn::parse(quote!(core::marker::Copy).into()).unwrap()),
            TypeAttributeBound::Custom(where_predicates) => where_predicates
        }
    }
}

#[derive(Clone)]
pub struct TypeAttribute {
    pub bound: TypeAttributeBound,
}

#[derive(Debug, Clone)]
pub struct TypeAttributeBuilder {
    pub enable_bound: bool,
}

impl TypeAttributeBuilder {
    pub fn from_clone_meta(&self, meta: &Meta) -> TypeAttribute {
        let mut bound = TypeAttributeBound::None;

        let correct_usage_for_clone_attribute = {
            let usage = vec![stringify!(#[educe(Clone)])];

            usage
        };

        let correct_usage_for_bound = {
            let usage = vec![stringify!(#[educe(Clone(bound))]), stringify!(#[educe(Clone(bound = "where_predicates"))]), stringify!(#[educe(Clone(bound("where_predicates")))])];

            usage
        };

        match meta {
            Meta::List(list) => {
                let mut bound_is_set = false;

                for p in list.nested.iter() {
                    match p {
                        NestedMeta::Meta(meta) => {
                            let meta_name = meta.name().to_string();

                            match meta_name.as_str() {
                                "bound" => {
                                    if !self.enable_bound {
                                        panic::unknown_parameter("Clone", meta_name.as_str());
                                    }

                                    match meta {
                                        Meta::List(list) => {
                                            for p in list.nested.iter() {
                                                match p {
                                                    NestedMeta::Literal(lit) => match lit {
                                                        Lit::Str(s) => {
                                                            if bound_is_set {
                                                                panic::reset_parameter(meta_name.as_str());
                                                            }

                                                            bound_is_set = true;

                                                            let where_predicates = create_where_predicates_from_lit_str(s);

                                                            bound = match where_predicates {
                                                                Some(where_predicates) => TypeAttributeBound::Custom(where_predicates),
                                                                None => panic::empty_parameter(meta_name.as_str())
                                                            };
                                                        }
                                                        _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                                    }
                                                    _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                                }
                                            }
                                        }
                                        Meta::NameValue(named_value) => {
                                            let lit = &named_value.lit;

                                            match lit {
                                                Lit::Str(s) => {
                                                    if bound_is_set {
                                                        panic::reset_parameter(meta_name.as_str());
                                                    }

                                                    bound_is_set = true;

                                                    let where_predicates = create_where_predicates_from_lit_str(s);

                                                    bound = match where_predicates {
                                                        Some(where_predicates) => TypeAttributeBound::Custom(where_predicates),
                                                        None => panic::empty_parameter(meta_name.as_str())
                                                    };
                                                }
                                                _ => panic::parameter_incorrect_format(meta_name.as_str(), &correct_usage_for_bound)
                                            }
                                        }
                                        Meta::Word(_) => {
                                            if bound_is_set {
                                                panic::reset_parameter(meta_name.as_str());
                                            }

                                            bound_is_set = true;

                                            bound = TypeAttributeBound::Auto;
                                        }
                                    }
                                }
                                _ => panic::unknown_parameter("Clone", meta_name.as_str())
                            }
                        }
                        _ => panic::attribute_incorrect_format("Clone", &correct_usage_for_clone_attribute)
                    }
                }
            }
            Meta::NameValue(_) => panic::attribute_incorrect_format("Clone", &correct_usage_for_clone_attribute),
            Meta::Word(_) => ()
        }

        TypeAttribute {
            bound,
        }
    }

    pub fn from_attributes(self, attributes: &[Attribute], traits: &[Trait]) -> TypeAttribute {
        let mut result = None;

        for attribute in attributes.iter() {
            let meta = attribute.parse_meta().unwrap();

            let meta_name = meta.name().to_string();

            match meta_name.as_str() {
                "educe" => match meta {
                    Meta::List(list) => {
                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.name().to_string();

                                    let t = Trait::from_str(meta_name);

                                    if let Err(_) = traits.binary_search(&t) {
                                        panic::trait_not_used(t.as_str());
                                    }

                                    if t == Trait::Clone {
                                        if result.is_some() {
                                            panic::reuse_a_trait(t.as_str());
                                        }

                                        result = Some(self.from_clone_meta(&meta));
                                    }
                                }
                                _ => panic::educe_format_incorrect()
                            }
                        }
                    }
                    _ => panic::educe_format_incorrect()
                }
                _ => ()
            }
        }

        result.unwrap_or(TypeAttribute {
            bound: TypeAttributeBound::None
        })
    }
}