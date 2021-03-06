# Contributing

This project is open for contributions. See the [issues](https://github.com/peterhuene/azure-functions-rs/issues) page for ideas on how you can help.

## Submitting Changes

To make changes to this project, please [fork the repo](https://help.github.com/en/articles/fork-a-repo). Create a feature branch with a name describing the feature added by the branch (i.e. `binding-to-iot`). Commit and push your changes to this branch. When you are ready to contribute, make your pull-request into the `dev` branch.

### Before you make a Pull Request

- [ ] You have pushed all your code to the remote repo.
- [ ] Your fork is up-to-date with the source repository (`peterhuene/azure-functions-rs`)
- [ ] All tests pass.
- [ ] "TODO" comments are removed.
- [ ] Temp variables are given good names.
- [ ] Any merge conflicts have the potential to be reasonably sorted out.

If you are uncertain about your contribution, that's ok! You can always make a [draft pull request](https://help.github.com/en/articles/about-pull-requests#draft-pull-requests).

## Repository Layout

This repository is split into multiple Rust crates:

- [azure-functions](https://github.com/peterhuene/azure-functions-rs/tree/master/azure-functions) - The `azure-functions` crate that defines the types and functions that are used when writing Azure Functions with Rust.
- [azure-functions-codegen](https://github.com/peterhuene/azure-functions-rs/tree/master/azure-functions-codegen) - The `azure-functions-codegen` crate that defines the procedural macros that are used when writing Azure Functions with Rust.
- [azure-functions-sdk](https://github.com/peterhuene/azure-functions-rs/tree/master/azure-functions-sdk) - The `azure-functions-sdk` crate that implements the `cargo func` command.
- [azure-functions-shared](https://github.com/peterhuene/azure-functions-rs/tree/master/azure-functions-shared) - The `azure-functions-shared` crate that defines types and functions that are shared between the `azure-functions-codegen` and `azure-functions` crates.
  - Note: the `azure-functions-shared/protobuf` directory is the git submodule for [Azure Functions Language Worker Protocol](https://github.com/Azure/azure-functions-language-worker-protobuf).
- [azure-functions-shared-codegen](https://github.com/peterhuene/azure-functions-rs/tree/master/azure-functions-shared-codegen) - The `azure-functions-shared-codegen` crate that defines the procedural macros used by the shared `azure-functions-shared` crate.
- [examples](https://github.com/peterhuene/azure-functions-rs/tree/master/examples) - The directory containing example Azure Functions.

## Setting Up a Dev Environment

### Cloning the Repository

This repository uses a git submodule for defining the [Azure Functions Language Worker Protocol](https://github.com/Azure/azure-functions-language-worker-protobuf).

Use `--recurse-submodules` when cloning this repository:

Cloning with SSH:

``` bash
git clone --recurse-submodules git@github.com:<GITHUB-USERNAME>/azure-functions-rs.git
```

Cloning with HTTPS:

``` bash
git clone --recurse-submodules https://github.com/<GITHUB-USERNAME>/azure-functions-rs.git
```

If you want to clone the source repository, replace `<GITHUB-USERNAME>` with `peterhuene`. To clone your own fork, replace the value with your GitHub username.

### Developing Your Contribution

Create a new branch from `dev` for the feature you are adding on your fork. Name the branch according to the feature you are adding (i.e. `binding-to-iot`). As you work on your contributions, code on the source repository may get updated. You can keep your fork up-to-date and avoid merge conflicts by adding the source repo as a remote, upstream branch.

``` bash
git remote add upstream https://github.com/peterhuene/azure-functions-rs
```

You only need to do this once. Then, to update your fork run

``` bash
git pull --rebase upstream <YOUR-FEATURE-BRANCH>
```

This will update your code with the changes that have occurred in the source repo.

> :warning: Note that this will not update the Azure Functions Language Worker Protocol. However, that code is much less likely to change.

### Building

Build at the root of the repository to build all the libraries and examples using `cargo build`:

``` bash
cargo build
```

### Running tests

Use `cargo test` to run the tests:

``` bash
cargo test
```

### Installing Azure Functions SDK from Source

To install the Azure Functions SDK from source, from the root of the project run  

``` bash
cargo install -f --path azure-functions-sdk
```