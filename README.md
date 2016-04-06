# rust-rosetta #
[![Linux Build Status](https://travis-ci.org/Hoverbear/rust-rosetta.png)](https://travis-ci.org/Hoverbear/rust-rosetta)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/xtm3xu8j4sou5jst/branch/master?svg=true)](https://ci.appveyor.com/project/Hoverbear/rust-rosetta/branch/master)
[![Coverage Status](https://coveralls.io/repos/Hoverbear/rust-rosetta/badge.svg?branch=master&service=github)](https://coveralls.io/github/Hoverbear/rust-rosetta?branch=master)

A repository for completing [this issue on rust-lang/rust](https://github.com/rust-lang/rust/issues/10513). This repository contains minimal working code for many simple (and not so simple) tasks. New contributors and learners of the language are welcome. We will try to work with you to make the code more idiomatic if you'd like!

Development is done on the `nightly` channel of Rust. You can get this using [`rustup`](rustup.rs).

This is a project for learning. If you're working on a problem and need some help? Drop by #rust-rosetta on [irc.mozilla.org](https://kiwiirc.com/client/irc.mozilla.org). *(Note: It's an asynchronous protocol, responses may be slow!)*

## Tasks Remaining ##

[List of Tasks Remaining](http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust)

> Important: Not all `rust-rosetta` tasks exist in their current form on Rosetta Code. Please cross-check with this repository before you start. Alternatively, check out [rust-rosetta coverage](https://euclio.github.io/rosetta-coverage) to see an automatically generated report of which tasks have been implemented where.

### Coverage ###

The `coverage` subcrate contains commands that are useful for discovering
incomplete solutions, or finding solutions that are different from the version
posted to the Rosetta Code wiki. To see what commands are available:

```sh
$ cd coverage
$ cargo run --release -- --help
```

## Tasks Complete ##

All tasks that have been completed are listed (along with a link to the problem) in [`Cargo.toml`](./Cargo.toml)

## Contributing ##
Looking to help out? Great, thanks! We have a few guidelines:

* The code you contribute *is* public domain.
* Don't be afraid of comments, the code is going to be written once, read hundreds of times, and maintained well past when you submit it.
* Keep your code as simple as possible, please avoid dead code warnings.

### Requirements ###
* Your code should build cleanly on latest nightly provided by [`rustup.sh`](http://doc.rust-lang.org/book/installing-rust.html)
* Please, use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) tool on your code.
If it's not possible, try to keep your contributions adherent to the official style guide which you can see at [this location](http://doc.rust-lang.org/nightly/style/). The style guide is still a work-in-progress, so there may be small differences.
* Include a link to the Rosetta Code Problem at the top of the code sample like this:

```rust
// http://rosettacode.org/wiki/FizzBuzz
```
* When implementing a new task, add the following to the `Cargo.toml` (It's alphabetical!):
If you'd like, you're welcome to add your contact details, name, or other information first.
Then add the entry in *lexicographical* ordering into [`Cargo.toml`](./Cargo.toml) like this:

```toml
[[bin]]
# http://rosettacode.org/wiki/FizzBuzz
name = "fizzbuzz"
path = "src/fizzbuzz.rs"
```

### Contributing process ###
Here's an idea of what a workflow would look like (in general-ish):

**If it's your first time**

* Choose a problem off Rosetta Code.
* Fork this repo on Github. ([Help here!](https://help.github.com/articles/fork-a-repo/))
* Clone your resulting repo onto your machine.
* When you contribute your first pull request, include yourself in the authors of the `Cargo.toml`!

**Every other time**

* Navigate to your `rust-rosetta` directory.
* Make sure you're on `master` branch.
* Update your fork ([Details](https://help.github.com/articles/syncing-a-fork/))
* Create a branch that is reasonably unique:
    - `git branch hoverbear-fizzbuzz`
* Switch to your newly created branch:
    - `git checkout hoverbear-fizzbuzz`
* Make your changes for this problem.
    - Add the new definition to the `Cargo.toml`
    - Add one code file with the appropriate name to the `src/` directory. If you need any data there is a separate folder for that.
    - Make sure to include unit tests for us, and comments! :)
* Stage your changes for commit, adding new and modified files to it:
    - `git add /src/new_file.rs`
    - `git add Cargo.toml`
* Check `git status` to make sure you don't mangle anything else.
* Commit your changes
    - `git commit -a -m "Implement blah blah blah"`
* Submit a [Pull request](https://help.github.com/articles/creating-a-pull-request/) here.

**After it's accepted**

* Delete the branch.

If this is unclear or you're having difficulties, just open an issue, we'd love to help.

## Testing ##

If you are contributing a problem to this repository please try to include a test so we can verify correctness. Here are some guidelines:

* Try to keep tests under 5 seconds on your machine (some problems require longer, that's fine). Do not include tests which may cause system instability (For example: forkbombs).
* The testing code should demonstrate invocation and result of the completed task, not the task itself.
  * For example, if the task takes parameters, the `#[test]` should create the necessary values and pass them in.
  * Remember to test for failure, too. I.E. If you're reading a file line by line, what happens if a file doesn't exist? One of Rust's biggest benefits is how it handles errors, show it off!
* Only talk to servers Rosetta code specifically directs you to.
* Do not download files unless that **is** the task. If that is the case make sure to delete the files afterwards.
* Do not execute anything unless that **is** the task. Do not execute anything as root.
* Do not depend on root privileges.
* Your testing code should be as simple as possible. Ideally it would look similar to this:

```rust
#[test]
fn test_file_exists {
    // ... Instantiate your necessary values (if there are enough to warrant it!) to pass in.
    let foo = 1;
    // ... Invoke your task and store the result.
    let result = my_task(foo);
    // ... Assert, unwrap, match, etc.
    assert_eq!(result, expected);
}
```
