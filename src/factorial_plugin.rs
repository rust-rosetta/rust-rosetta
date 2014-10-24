#![crate_type="dylib"]
#![feature(plugin_registrar)]

// basic syntax extension to calculate the factorial of 10
// at compile time for the compile-time calculation
// task (the task itself is in compile_time_calculation.rs)
extern crate syntax;
extern crate rustc;

use syntax::codemap::Span;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, MacExpr};
use syntax::ext::build::AstBuilder;
use rustc::plugin::Registry;
use std::iter::range_inclusive;

// this is based on the example from http://doc.rust-lang.org/guide-plugin.html
fn exp_factorial(cx: &mut ExtCtxt, sp: Span, _: &[TokenTree])
        -> Box<MacResult + 'static> {
    // calculating the actual 10!
    let result = range_inclusive(1u, 10).fold(1, |accum, elem| accum * elem);    

    MacExpr::new(cx.expr_uint(sp, result))
}

// here's where we register the macro with the name of factorial_10
// so that it can be invoked a with every other macro with a ! at the end
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("factorial_10", exp_factorial);
}