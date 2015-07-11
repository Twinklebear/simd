#![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;

pub mod backend;
pub mod simd;

#[test]
fn it_works() {
}

