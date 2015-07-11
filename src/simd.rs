use syntax::ast;
use syntax::codemap::Span;
use syntax::ptr::P;
use syntax::parse::token;
use syntax::ext::expand;
use syntax::ext::base::ExtCtxt;
use syntax::ext::base::SyntaxExtension;
use syntax::fold::Folder;
use rustc::plugin::Registry;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("simd"), SyntaxExtension::Modifier(Box::new(simd)));
}

fn simd<'a>(cx: &mut ExtCtxt<'a>, sp: Span, mi: &ast::MetaItem, orig_item: P<ast::Item>)
            -> P<ast::Item> {
    println!("plugin runnin!");
    orig_item
}

