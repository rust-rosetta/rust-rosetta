#![feature(rustc_private)]

extern crate arena;

use arena::TypedArena;

fn main() {
    // Memory is allocated using the default allocator (currently jemalloc).  The memory is
    // allocated in chunks, and when one chunk is full another is allocated.  This ensures that
    // references to an arena don't become invalid when the original chunk runs out of space.  The
    // chunk size is configurable as an argument to TypedArena::with_capacity if necessary.
    let arena = TypedArena::new();
    // The arena crate contains two types of arenas: TypedArena and Arena.  Arena is
    // reflection-basd and slower, but can allocate objects of any type.  TypedArena is faster, and
    // can allocate only objects of one type.  The type is determined by type inference--if you try
    // to allocate an integer, then Rust's compiler knows it is an integer arena.
    let v1 = arena.alloc(1i32);

    // TypedArena returns a mutable reference
    let v2 = arena.alloc(3);
    *v2 += 38;
    println!("{}", *v1 + *v2);

    // The arena's destructor is called as it goes out of scope, at which point it deallocates
    // everything stored within it at once.
}
