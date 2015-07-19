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
    let supported = match orig_item.node {
        ast::ItemFn(_, _, _, _, ref generics, _) => !generics.is_parameterized(),
        _ => false,
    };
    if !supported {
        cx.span_err(orig_item.span, "#[simd] not supported on this item");
        return orig_item;
    }

    println!("item: {:#?}", orig_item);
    orig_item.map(|item| {
        let new_node = match item.node {
            ast::ItemFn(decl, safety, constness, abi, generics, block) => {
                println!("Function declaration:");
                for a in &decl.inputs {
                    println!("{:#?}", a);
                    println!("{:#?}", a.ty.node);
                    match a.ty.node {
                        ast::TyPath(_, ref p) => {
                            println!("path = {:#?}", p);
                            println!("segments = {:#?}", p.segments);
                        },
                        _ => {},
                    }
                }
                println!("Return type = {:#?}", decl.output);
                println!("\nFunction statements:");
                for i in &block.stmts {
                    println!("{:#?}", i);
                    match i.node {
                        ast::StmtDecl(ref decl, _) => {
                            match decl.node {
                                ast::DeclLocal(ref local) => {
                                    match local.ty {
                                        Some(ref ty) => println!("type {:#?}", ty),
                                        None => println!("no type"),
                                    }
                                },
                                ast::DeclItem(_) => {
                                    println!("item decl");
                                },
                            }
                        },
                        _ => println!("other decl"),
                    }
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
                ast::ItemFn(decl, safety, constness, abi, generics, block)
            },
            _ => unreachable!(),
        };
        ast::Item {
            node: new_node,
            .. item
        }
    })
}

