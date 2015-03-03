#![crate_type="dylib"]
#![feature(plugin_registrar)]
#![feature(rustc_private)]
// basic syntax extension to calculate the factorial of 10
// at compile time for the compile-time calculation
// task (the task itself is in compile_time_calculation.rs)
extern crate syntax;
extern crate rustc;

use syntax::ast;
use syntax::codemap::Span;
use syntax::parse;
use syntax::parse::token;
use syntax::ast::TokenTree;
use syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use syntax::ext::build::AstBuilder;  // trait for expr_usize
use rustc::plugin::Registry;

fn exp_factorial(cx: &mut ExtCtxt, sp: Span, tts: &[TokenTree])
        -> Box<MacResult + 'static> {
    // extract the argument and ensure there is only one and it's a usize
    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());

    // Try to parse a literal (doesn't need to be a number)
    let literal = if parser.token != token::Eof {
        parser.parse_lit()
    } else {
        // span_err shows a compile time error to the user
        cx.span_err(sp, "Unexpected end of file");
        return DummyResult::any(sp);
    };

    // Take the number of the literal (if it is a number)
    let n: u64 = match literal.node {
        ast::LitInt(x, _) => x,
         _              => {
            cx.span_err(sp, "Invalid literal (expected unsigned integer)");
            return DummyResult::any(sp);
        }
    };

    // calculating the factorial
    let result = (1..(n as usize)+1).fold(1, |accum, elem| accum * elem);

    MacEager::expr(cx.expr_usize(sp, result))
}

// here's where we register the macro with the name of factorial_10
// so that it can be invoked a with every other macro with a ! at the end
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("factorial", exp_factorial);
}
