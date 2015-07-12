use syntax::ast;
use syntax::ast::Item;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ext::expand;
use syntax::ext::base::{ExtCtxt, SyntaxExtension};
use syntax::fold::Folder;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("simd"), SyntaxExtension::Modifier(Box::new(simd)));
}

fn simd<'a>(cx: &mut ExtCtxt<'a>, sp: Span, mi: &ast::MetaItem, orig_item: P<ast::Item>)
            -> P<ast::Item> {
    println!("plugin runnin!");
    println!("item: {:#?}", orig_item);
    match orig_item.node {
        ast::ItemFn(ref decl, _, _, _, _, ref block) => {
            println!("Function statements:");
            for i in &block.stmts {
                println!("{:#?}", i);
            }
            if let Some(ref expr) = block.expr {
                println!("Function has an expression");
                println!("{:#?}", expr);
                println!("node = {:#?}", expr.node);
                match expr.node {
                    ast::ExprBinary(ref op, ref l, ref r) => {
                        println!("left = {:#?}", l.node);
                        println!("right = {:#?}", r.node);
                    },
                    _ => {},
                }
            }
        },
        _ => {},
    }
    orig_item
}

