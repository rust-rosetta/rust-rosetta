#![cfg_attr(feature="clippy", allow(blacklisted_name))]

struct Foo;

impl Foo {
    // implementation of an instance method for struct Foo
    // returning the answer to life
    fn get_the_answer_to_life(&self) -> i32 {
        42
    }

    // implementation of a static method for struct Foo
    // returning a new instance object
    fn new() -> Foo {
        println!("Hello, world!");
        Foo // returning the new Foo object
    }
}

impl Default for Foo {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    // create the instance object foo,
    // by calling the static method new of struct Foo
    let foo = Foo::new();

    // get the answer to life
    // by calling the instance method of object foo
    println!("The answer to life is {}.", foo.get_the_answer_to_life());
}
