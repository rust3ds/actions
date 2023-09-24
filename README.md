# test-runner-3ds
<!-- TODO: better name! -->

A set of tools for running automated Rust tests against Citra (3DS emulator).

## Components

* `test-runner`: a Rust crate for writing tests for 3DS homebrew
* GitHub Actions:
  * `setup`: action for setting up the Rust 3DS toolchain in workflows
  * `run-tests`: action for running test executables with Citra in workflows

## Usage

First the test runner to your crate:

```sh
cargo add --dev test-runner --git https://github.com/ian-h-chamberlain/test-runner-3ds
```

In `lib.rs` and any integration test files:

```rs
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
```

Then use the `setup` and `run-tests` actions in your github workflow. This
example shows the default value for each of the inputs:

```yml
jobs:
  test:
    runs-on: ubuntu-latest
    container:
      image: devkitpro/devkitarm
      volumes:
        # This is required so the test action can `docker run` the runner:
        - '/var/run/docker.sock:/var/run/docker.sock'
        # This is required so doctest artifacts are accessible to the action:
        - '/tmp:/tmp'

    steps:
      - name: Checkout branch
        uses: actions/checkout@v4

      - name: Setup Rust3DS toolchain
        uses: ian-h-chamberlain/test-runner-3ds/setup@v1
        with:
          # Optionally use a more specific nightly toolchain here if desired
          toolchain: nightly

      - name: Build and run tests
        uses: ian-h-chamberlain/test-runner-3ds/run-tests@v1
        with:
          # Optionally add arguments to pass to `cargo 3ds test`
          args: ''
          # Optionally set the name of the built test-runner docker image
          runner-image: test-runner-3ds
          # Optionally change to a given directory before running tests. Note
          # that this should use the environment variable ${GITHUB_WORKSPACE}
          # rather than ${{ github.workspace }} to avoid the issue described in
          # https://github.com/actions/runner/issues/2058
          working-directory: ${GITHUB_WORKSPACE}
```
