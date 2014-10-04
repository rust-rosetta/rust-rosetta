# rust-rosetta #
[![Build Status](https://travis-ci.org/Hoverbear/rust-rosetta.png)](https://travis-ci.org/Hoverbear/rust-rosetta)

A repository for completing [this issue on mozilla/rust](https://github.com/mozilla/rust/issues/10513). This repository contains minimal working code for many simple (and not so simple) tasks. New contributors and learners of the language are welcome. We will try to work with you to make the code more idiomatic over time.

> Working on a problem, need some help? Drop by #rust-rosetta on irc.mozilla.org.
Get in touch with @Hoverbear if you need help setting up.

## Tasks Remaining ##

[List of Tasks Remaining](http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust)

## Tasks Complete ##

All tasks that have been completed are listed (along with a link to the problem) in [`Cargo.toml`](./Cargo.toml)

## Contributing ##
Looking to help out? Great, thanks! We have a few guidelines:

* The code you contribute *is* public domain.
* Your code should build cleanly on the `master` branch of Rust.
* Keep your code as simple as possible, please avoid Dead Code warnings.
* Don't be afraid of comments, the code is going to be written once, read hundreds of times, and maintained until past the 1.0 release of Rust.
* Include a link to the Rosetta Code Problem at the top of the code sample.
* Add a line to the Readme section below. (It's alphabetical!)
* Unit tests are strongly encouraged. Having troubles getting the build to work? Check about [Not Test Flags](https://github.com/Hoverbear/rust-rosetta/pull/96#issuecomment-43816696)

If you have unit-tests, you will need to mark `fn main`, and possibly
some other `fn`s with `#[cfg(not(test))]` to avoid dead-code warnings.
The reason dead-code warnings appear is because `main` is never called
during a test.

```rust
#[cfg(not(test))]
fn main() {
    println!("I need to be not compiled during tests");
}

#[test]
fn test_me() {
    assert!(true);
}
```

If you are unable to test your program then mark the entire file with
`// not_tested`.  This will disable testing completely for that
file.

```rust
// not_tested

fn main(){
    println!("Please add unit-tests later.");
}
```

The top of your code should look like this:

```rust
// http://rosettacode.org/wiki/foo
```
If you'd like, you're welcome to add your contact details, name, or other information as well.

[Contributors](https://github.com/Hoverbear/rust-rosetta/graphs/contributors)

## Beginners Guide to Contributing ##
If you look [here](https://github.com/Hoverbear/rust-rosetta/network) you can see how most contributions "merge" over time with the main tree. People will create multiple branches off the same main repo. So you see your long one? Instead of multiple branches coming and going off the main repo, it's one long one.

Here's an idea of what a workflow would look like (in general-ish):

**If it's your first time**

* Choose a problem off Rosetta Code.
* Fork this repo on Github. ([Help here!](https://help.github.com/articles/fork-a-repo))
* Clone your resulting repo onto your machine.

**Every other time**

* Navigate to your `rust-rosetta` directory.
* Make sure you're on `master` branch.
* Update your fork ([Details](https://help.github.com/articles/syncing-a-fork))
* Create a branch that is reasonably unique `Ein06-func-impl` is a good example.
* Make your changes for this problem.
    - Add the new definition to the README.md
    - Add one code file with the appropriate name to the `src/` directory. If you need any data there is a separate folder for that.
    - Make sure to include unit tests for us, and comments! :)
* Check `git status` to make sure you don't mangle anything else.
* Commit your changes (`git commit -a -m "Implement blah blah blah"`)
* Submit a [Pull request](https://help.github.com/articles/creating-a-pull-request) here.

**After it's accepted**

* Delete the branch.

If this is unclear or you're having difficulties, just open an issue, we'd love to help.
