extern crate proc_macro;

mod schema_module;

use proc_macro::TokenStream;
use quote::quote;
use syn::braced;
use syn::parse::Result;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;
use syn::Token;
use syn::Type;
use syn::{ExprArray, Ident};

use crate::schema_module::parse_input_schema_module;

#[proc_macro]
pub fn schema_module(item: TokenStream) -> TokenStream {
    let (module_name_ident, schema_entities) = parse_input_schema_module(item);

    let entity_keys = schema_entities
        .iter()
        .map(|entity| entity.cased_key())
        .map(|key| syn::parse_str::<Ident>(&key).unwrap())
        .collect::<Vec<_>>();
    let entity_cids = schema_entities
        .iter()
        .map(|entity| entity.entity_cid())
        .map(|key| syn::parse_str::<ExprArray>(&key).unwrap())
        .collect::<Vec<_>>();
    let entity_kind_idents = schema_entities
        .iter()
        .map(|entity| entity.entity_kind_ident())
        .collect::<Vec<_>>();
    let entity_json_strs = schema_entities
        .iter()
        .map(|entity| entity.entity_json_str())
        .collect::<Vec<_>>();

    let tokens = quote! {
        mod #module_name_ident {
            use rlay_ontology::prelude::*;

            pub mod cids {
                #(
                    pub static #entity_keys : &'static [u8] = &#entity_cids;
                )*
            }

            #(
                pub fn #entity_keys() -> #entity_kind_idents {
                    let entity: FormatWeb3<#entity_kind_idents> = serde_json::from_str(#entity_json_strs).unwrap();
                    entity.0
                }
            )*
        }
    };
    tokens.into()
}

#[derive(Debug)]
struct IndividualWithChildren {
    // struct_token: Token![struct],
    // ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

#[derive(Debug)]
struct Field {
    name: Ident,
    colon_token: Token![:],
    ty: Type,
}

impl Parse for IndividualWithChildren {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(IndividualWithChildren {
            // struct_token: input.parse()?,
            // ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse)?,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

// #[proc_macro]
// pub fn individual_with_children(item: TokenStream) -> TokenStream {
// let map_group: IndividualWithChildren = syn::parse(item).unwrap();
// dbg!(&map_group);
// todo!()
// }
