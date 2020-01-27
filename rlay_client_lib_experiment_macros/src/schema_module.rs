use cid_fork_rlay::ToCid;
use heck::SnekCase;
use proc_macro::TokenStream;
use rlay_ontology::ontology::web3::FormatWeb3;
use rlay_ontology::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use syn::parse::Parser;
use syn::{punctuated::Punctuated, Expr, Ident, Token};

#[derive(Deserialize, Debug)]
pub struct SchemaEntity {
    key: String,
    assertion: Value,
}

impl SchemaEntity {
    pub fn cased_key(&self) -> String {
        self.key.to_snek_case()
    }

    pub fn entity(&self) -> Entity {
        let entity: FormatWeb3<Entity> = serde_json::from_value(self.assertion.clone()).unwrap();
        entity.0
    }

    pub fn entity_kind_ident(&self) -> Ident {
        let entity_kind_str: &str = self.entity().kind().into();
        syn::parse_str(entity_kind_str).unwrap()
    }

    /// Returns entity cid formated as a bytes slice (without reference prefix):
    /// e.g.: [12, 0, 3]
    pub fn entity_cid(&self) -> String {
        let cid_bytes: Vec<u8> = self.entity().to_cid().unwrap().to_bytes();
        let rendered_cid = format!("{:?}", cid_bytes);

        rendered_cid
    }

    pub fn entity_json_str(&self) -> String {
        serde_json::to_string(&self.assertion).unwrap()
    }
}

pub fn parse_input_schema_module(item: TokenStream) -> (Ident, Vec<SchemaEntity>) {
    let parser = Punctuated::<Expr, Token![,]>::parse_terminated;
    let mut parts = parser.parse(item).unwrap().into_iter();
    let module_name = match parts.next().unwrap() {
        Expr::Path(inner) => inner,
        _ => panic!(),
    };
    let schema_path = match parts.next().unwrap() {
        Expr::Lit(inner) => inner,
        _ => panic!(),
    }
    .lit;

    let module_name_ident = module_name.path.get_ident().unwrap();
    let schema_path = match schema_path {
        syn::Lit::Str(inner) => inner,
        _ => panic!(),
    }
    .value();

    let schema_contents = std::fs::read_to_string(schema_path).unwrap();
    let schema_entities: Vec<SchemaEntity> = serde_json::from_str(&schema_contents).unwrap();
    (module_name_ident.to_owned(), schema_entities)
}
