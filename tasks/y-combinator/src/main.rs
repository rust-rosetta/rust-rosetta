use std::sync::Arc;

// Arc<Box<Closure>>
macro_rules! abc {
    ($x:expr) => (Arc::new(Box::new($x)));
}

#[derive(Clone)]
enum Mu<T> {
    Roll(Arc<Box<Fn(Mu<T>) -> T>>),
}

fn unroll<T>(Mu::Roll(f): Mu<T>) -> Arc<Box<Fn(Mu<T>) -> T>> {
    f.clone()
}

pub type Func<A> = Arc<Box<Fn(A) -> A>>;
pub type RecFunc<A> = Arc<Box<Fn(Func<A>) -> Func<A>>>;

pub fn y<A: 'static>(f: RecFunc<A>) -> Func<A> {
    let g: Arc<Box<Fn(Mu<Func<A>>) -> Func<A>>> = abc!(move |x: Mu<Func<A>>| -> Func<A> {
        let f = f.clone();
        abc!(move |a: A| -> A {
            let f = f.clone();
            f(unroll(x.clone())(x.clone()))(a)
        })
    });
    g(Mu::Roll(g.clone()))
}

#[macro_export]
macro_rules! y {
    (|$name:ident| $fun:tt) => {
        y(abc!(|$name| abc!($fun)))
    }
}

fn fac(n: u32) -> u32 {
    let fn_: Func<u32> = y!(|f| (move |x| if x == 0 { 1 } else { f(x - 1) * x }));
    fn_(n)
}

fn fib(n: u32) -> u32 {
    let fn_: Func<u32> = y!(|f| (move |x| if x < 2 { x } else { f(x - 1) + f(x - 2) }));
    fn_(n)
}

fn main() {
    println!("{}", fac(10));
    println!("{}", fib(10))
}

#[cfg(test)]
mod tests {
    #[test]
    fn fac() {
        assert_eq!(super::fac(10), 3628800);
    }

    #[test]
    fn fib() {
        assert_eq!(super::fib(10), 55);
    }
}
