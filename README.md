# rust-rosetta #
[![Build Status](https://travis-ci.org/Hoverbear/rust-rosetta.png)](https://travis-ci.org/Hoverbear/rust-rosetta)

A repository for completing [this issue on mozilla/rust](https://github.com/mozilla/rust/issues/10513). This repository contains minimal working code for many simple (and not so simple) tasks. New contributors and learners of the language are welcome. We will try to work with you to make the code more idiomatic over time.

> Working on a problem, need some help? Drop by #rust-rosetta on irc.mozilla.org.

## Tasks Remaining ##

[List of Tasks Remaining](http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust)

> Important: Not all `rust-rosetta` tasks exist in their current form on Rosetta Code. Please double check before you start.

## Tasks Complete ##

All tasks that have been completed are listed (along with a link to the problem) in [`Cargo.toml`](./Cargo.toml)

## Contributing ##
Looking to help out? Great, thanks! We have a few guidelines:

* The code you contribute *is* public domain.
* Your code should build cleanly latest nightly provided by [`rustup.sh`](http://doc.rust-lang.org/guide.html#installing-rust)
* Keep your code as simple as possible, please avoid Dead Code warnings.
* Don't be afraid of comments, the code is going to be written once, read hundreds of times, and maintained until past the 1.0 release of Rust.
* Include a link to the Rosetta Code Problem at the top of the code sample.
* Add a line to the `Cargo.toml` below. (It's alphabetical!)


The top of your code should look like this:

```rust
// http://rosettacode.org/wiki/foo
```
If you'd like, you're welcome to add your contact details, name, or other information as well.

Then add the entry in *lexicographical* ordering into [`Cargo.toml`](./Cargo.toml) like this:

```toml
[[bin]]
# http://rosettacode.org/wiki/Hailstone_sequence
name = "hailstone"
path = "src/hailstone.rs"
```

## Contributing ##

Here's an idea of what a workflow would look like (in general-ish):

**If it's your first time**

* Choose a problem off Rosetta Code.
* Fork this repo on Github. ([Help here!](https://help.github.com/articles/fork-a-repo))
* Clone your resulting repo onto your machine.
* When you contribute your first pull request, include yourself in the authors of the `Cargo.toml`!

**Every other time**

* Navigate to your `rust-rosetta` directory.
* Make sure you're on `master` branch.
* Update your fork ([Details](https://help.github.com/articles/syncing-a-fork))
* Create a branch that is reasonably unique:
    - `git branch hoverbear-hailstone`
* Make your changes for this problem.
    - Add the new definition to the `Cargo.toml`
    - Add one code file with the appropriate name to the `src/` directory. If you need any data there is a separate folder for that.
    - Make sure to include unit tests for us, and comments! :)
* Check `git status` to make sure you don't mangle anything else.
* Commit your changes 
    - `git commit -a -m "Implement blah blah blah"`
* Submit a [Pull request](https://help.github.com/articles/creating-a-pull-request) here.

**After it's accepted**

* Delete the branch.

If this is unclear or you're having difficulties, just open an issue, we'd love to help.
