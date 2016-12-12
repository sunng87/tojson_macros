#![feature(proc_macro, proc_macro_lib)]

#[macro_use]
extern crate quote;

extern crate proc_macro;
extern crate syn;

use syn::Body::Struct;
use syn::VariantData;
use quote::Tokens;
use proc_macro::TokenStream;

#[proc_macro_derive(ToJson)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    // Parse a string of items to an AST
    let ast = syn::parse_macro_input(&source).unwrap();

    let name = &ast.ident;
    let generics = &ast.generics;
    let where_clause = &ast.generics.where_clause;

    let fields = match ast.body {
        Struct(VariantData::Struct(ref fields)) => fields,
        _ => {
            panic!("#[derive(ToJson)] is only valid for struct");
        }
    };

    let field_to_json: Vec<Tokens> =
        fields.iter()
              .map(|ref f| {
                  let field_name = f.ident.as_ref().unwrap();
                  let field_name_str = field_name.to_string();
                  quote!(__container.insert(#field_name_str.to_owned(), self.#field_name.to_json()))
              })
              .collect();

    let res = quote! {
        impl #generics ::rustc_serialize::json::ToJson for #name #generics #where_clause {
            fn to_json(&self) -> ::rustc_serialize::json::Json {
                let mut __container = ::std::collections::BTreeMap::new();
                #(#field_to_json;)*
                ::rustc_serialize::json::Json::Object(__container)
            }
        }
    };


    res.to_string().parse().unwrap()
}
