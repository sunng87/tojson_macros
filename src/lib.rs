#![feature(custom_derive, quote, plugin, plugin_registrar, rustc_private)]

#[macro_use]
extern crate rustc;
#[macro_use]
extern crate syntax;

use syntax::ext::base::MultiDecorator;
use syntax::parse::token;
use syntax::ast;
use syntax::codemap::Span;
use syntax::ext::base::{ExtCtxt, Annotatable};
use syntax::ptr::P;
use rustc::plugin::Registry;

fn expand_derive_tojson(ct: &mut ExtCtxt, span: Span, _: &ast::MetaItem,
                        item: &Annotatable, push: &mut FnMut(Annotatable)) {
    if let Annotatable::Item(ref item) = *item {
        if let ast::ItemStruct(ref struct_def, _) = item.node {
            let struct_name = item.ident;
            let conv_body: Vec<P<ast::Expr>> = struct_def.fields().iter().map(|field| {
                if let ast::NamedField(name, _) = field.node.kind {
                    let name_str = name.name.as_str();
                    quote_expr!(ct, {
                        __container.insert($name_str.to_string(), self.$name.to_json());
                    })
                } else {
                    ct.span_fatal(span, "#[derive(ToJson)] doesn't support simple struct for now");
                }
            }).collect();
            let impl_item = quote_item!(ct,
                                        #[automatically_derived]
                                        impl ::rustc_serialize::json::ToJson for $struct_name {
                                            fn to_json(&self) -> ::rustc_serialize::json::Json {
                                                let mut __container = ::std::collections::BTreeMap::new();
                                                $conv_body;
                                                ::rustc_serialize::json::Json::Object(__container)
                                            }
                                        }).unwrap();

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
