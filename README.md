# test-runner-3ds
<!-- TODO: better name! -->

A set of tools for running automated Rust tests against Citra (3DS emulator).

## Components

* `test-runner`: a Rust crate for writing tests for 3DS homebrew
* `Dockerfile`: builds a container for running test executables with Citra.
* GitHub Actions:
  * `.github/actions/setup`: action for setting up the Rust 3DS toolchain in
    workflows
  * `.github/actions/citra`: action for running test executables with Citra in
    workflows

<!-- TODO: usage section for github actions -->
