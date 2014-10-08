// Implements http://rosettacode.org/wiki/Arena_storage_pool
extern crate arena;

use arena::TypedArena;

#[cfg(not(test))]
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
    // Note that the current implementation of TypedArena returns a & (shared) reference, not a
    // mutable reference.  Therefore, unless the type has interior mutability (like Cell) it is not
    // modifiable after it is allocated.  This is to prevent the reference from being dropped
    // accidentally before the arena is deallocated.
    let v2 = arena.alloc(3);
    println!("{}", v1 + *v2);
    // The arena's destructor is called as it goes out of scope, at which point it deallocates
    // everything stored within it at once.
}
