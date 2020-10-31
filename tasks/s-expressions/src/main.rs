use s_expressions::{ParseContext, SExp, SEXP_STRING_IN, SEXP_STRUCT};

fn main() {
    println!("{:?}", SEXP_STRUCT.buffer_encode());
    let ctx = &mut ParseContext::new(SEXP_STRING_IN);
    println!("{:?}", SExp::parse(ctx));
}
