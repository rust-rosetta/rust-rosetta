//! A module level comment

// A single line comment

/*
 *  This is a multi-line (aka block) comment
 *
 *  /*
 *   *  containing nested multi-line comment
 *   *  (nesting supported since 0.9-pre https://github.com/rust-lang/rust/issues/9468)
 *   */
 */

/// Outer single line Rustdoc comments apply to the next item.

/**
 *  Outer multi-line Rustdoc comments.
 *
 *  Leading asterisk (*) in multi-line Rustdoc comments
 *  is not considered to be part of the comment text,
 *  blanks and tabs preceding the initial asterisk (*) are also stripped.
 */
fn example1() {
    //! Inner single line Rustdoc comments apply to their enclosing item.

    /*!
     *  Inner multi-line Rustdoc comments.
     *  See also https://doc.rust-lang.org/book/documentation.html
     */
}

#[doc = "Unsugared outer Rustdoc comments.
        (outer attributes are not terminated by a semi-colon)"]
fn example2() {
    #![doc = "Unsugared inner Rustdoc comments.
              See also https://doc.rust-lang.org/book/documentation.html"]
}

fn main() {
    example1();
    example2();
}
