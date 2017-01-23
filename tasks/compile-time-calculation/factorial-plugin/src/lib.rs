#![feature(plugin_registrar)]
#![feature(rustc_private)]

//! basic syntax extension to calculate the factorial of 10 at compile time for the compile-time
//! calculation task (the task itself is in `src/compile_time_calculation.rs`)
extern crate syntax;
extern crate rustc_plugin;

use syntax::ast;
use syntax::codemap::Span;
use syntax::parse;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use syntax::tokenstream::TokenTree;
use rustc_plugin::Registry;

fn exp_factorial(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree]) -> Box<MacResult + 'static> {
    // extract the argument and ensure there is only one and it's a usize
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), tts.to_vec());

    // Try to parse a literal (doesn't need to be a number)
    let literal = match parser.parse_lit() {
        Ok(l) => l,
        Err(_) => {
            // span_err shows a compile time error to the user
            cx.span_err(sp, "fatal error");
            return DummyResult::any(sp);
        }
    };

    // Take the number of the literal (if it is a number)
    let n = match literal.node {
        ast::LitKind::Int(x, _) => x,
        _ => {
            cx.span_err(sp, "Invalid literal (expected unsigned integer)");
            return DummyResult::any(sp);
        }
    };

    // calculating the factorial
    let result = (1..(n as usize) + 1).fold(1, |accum, elem| accum * elem);

    MacEager::expr(cx.expr_usize(sp, result))
}

/// here's where we register the macro with the name of `factorial` so that it can be invoked a
/// like every other macro with a ! at the end
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("factorial", exp_factorial);
}
