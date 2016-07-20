# rust-rosetta #
[![Linux Build Status](https://travis-ci.org/Hoverbear/rust-rosetta.png)](https://travis-ci.org/Hoverbear/rust-rosetta)
[![Windows Build status](https://ci.appveyor.com/api/projects/status/xtm3xu8j4sou5jst/branch/master?svg=true)](https://ci.appveyor.com/project/Hoverbear/rust-rosetta/branch/master)
[![Coverage Status](https://coveralls.io/repos/Hoverbear/rust-rosetta/badge.svg?branch=master&service=github)](https://coveralls.io/github/Hoverbear/rust-rosetta?branch=master)

A repository for completing [this issue on rust-lang/rust](https://github.com/rust-lang/rust/issues/10513). This repository contains minimal working code for many simple (and not so simple) tasks. New contributors and learners of the language are welcome. We will try to work with you to make the code more idiomatic if you'd like!

Development is done on the `nightly` channel of Rust. You can get this using [`rustup`](https://www.rustup.rs/).

This is a project for learning. If you're working on a problem and need some help? Drop by #rust-rosetta on [irc.mozilla.org](https://kiwiirc.com/client/irc.mozilla.org). *(Note: It's an asynchronous protocol, responses may be slow!)*

## Tasks Remaining ##

[List of Tasks Remaining](http://rosettacode.org/wiki/Reports:Tasks_not_implemented_in_Rust)

> Important: Not all `rust-rosetta` tasks exist in their current form on Rosetta Code. Please cross-check with this repository before you start. Alternatively, check out [rust-rosetta coverage](https://euclio.github.io/rosetta-coverage) to see an automatically generated report of which tasks have been implemented where.

### Coverage ###

The main crate contains a `coverage` binary that is useful for discovering
incomplete solutions, or finding solutions that are different from the version
posted to the Rosetta Code wiki. To see what commands are available:

```sh
$ cargo run --release --bin coverage -- --help
```

## Tasks Complete ##

All tasks that have been completed are listed (along with a link to the problem) in [`Cargo.toml`](./Cargo.toml)

## Contributing ##

Looking to contribute? Great! Take a look at [CONTRIBUTING.md](CONTRIBUTING.md) to get started.
