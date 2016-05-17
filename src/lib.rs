#![feature(custom_derive, quote, plugin, plugin_registrar, rustc_private)]

#[macro_use]
extern crate rustc_plugin;
#[macro_use]
extern crate syntax;

use syntax::ext::base::MultiDecorator;
use syntax::parse::token;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::build::AstBuilder;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ptr::P;
use rustc_plugin::Registry;

fn expand_derive_tojson(ct: &mut ExtCtxt, span: Span, _: &ast::MetaItem,
                        item: &Annotatable, push: &mut FnMut(Annotatable)) {
    if let Annotatable::Item(ref item) = *item {
        if let ast::ItemKind::Struct(ref struct_def, ref generics) = item.node {
            let struct_name = item.ident;

            let lifetimes: Vec<ast::Lifetime> = generics.lifetimes.iter().map(|ld| ld.lifetime).collect();
            let generic_parameters: Vec<P<ast::Ty>> = generics.ty_params.iter().map(|ty| ct.ty_ident(span, ty.ident)).collect();
            let struct_ty = ct.ty_path(ct.path_all(span, false, vec![struct_name], lifetimes, generic_parameters, Vec::new()));
            let where_clause = generics.clone().where_clause;

            let conv_body: Vec<P<ast::Expr>> = struct_def.fields().iter().map(|field| {
                if let Some(ident) = field.ident {
                    let name_str = ident.name.as_str();
                    quote_expr!(ct, {
                        __container.insert($name_str.to_owned(), self.$ident.to_json());
                    })
                } else {
                    ct.span_fatal(span, "#[derive(ToJson)] doesn't support simple struct for now");
                }
            }).collect();
            let impl_item = quote_item!(ct,
                                        impl $generics ::rustc_serialize::json::ToJson for $struct_ty $where_clause {
                                            fn to_json(&self) -> ::rustc_serialize::json::Json {
                                                let mut __container = ::std::collections::BTreeMap::new();
                                                $conv_body;
                                                ::rustc_serialize::json::Json::Object(__container)
                                            }
                                        }).unwrap();

            //println!("{}", syntax::print::pprust::item_to_string(&impl_item));
            push(Annotatable::Item(impl_item));
        } else {
            ct.span_err(span, "#[derive(ToJson)] is only valid for struct");
        }
    } else {
        ct.span_err(span, "#[derive(ToJson)] is only valid for struct");
    }

}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(
        token::intern("derive_ToJson"),
        MultiDecorator(Box::new(expand_derive_tojson)));
}
