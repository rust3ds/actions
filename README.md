# test-runner

A set of tools for running automated Rust tests against Citra (3DS emulator).

## Components

* `test-runner`: a Rust crate for writing tests for 3DS homebrew
* GitHub Actions:
  * `setup`: action for setting up the Rust 3DS toolchain in workflows
  * `run-tests`: action for running test executables with Citra in workflows

## Usage

First the test runner to your crate:

```sh
cargo add --dev test-runner --git https://github.com/rust3ds/test-runner
```

In `lib.rs` and any integration test files:

```rs
#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
```

Then use the `setup` and `run-tests` actions in your github workflow. This
example shows the default value for each of the inputs.

```yml
jobs:
  test:
    runs-on: ubuntu-latest
    container:
      image: devkitpro/devkitarm
      volumes:
        # This is required so the test action can `docker run` the runner:
        - '/var/run/docker.sock:/var/run/docker.sock'

    steps:
      - name: Checkout branch
        uses: actions/checkout@v4

      - name: Setup Rust3DS toolchain
        uses: rust3ds/test-runner/setup@v1
        with:
          # Optionally use a more specific nightly toolchain here if desired
          toolchain: nightly

      - name: Build and run tests
        uses: rust3ds/test-runner/run-tests@v1
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

See [`ci.yml`](.github/workflows/ci.yml) to see a full lint and test workflow
using these actions (including uploading output artifacts from the tests).

## Caveats

* GDB doesn't seem to support separate output streams for `stdout` and `stderr`,
  so all test output to `stderr` will end up combined with `stdout` and both will be
  printed to the runner's `stdout`. If you know a workaround for this that doesn't
  require patching + building GDB itself please open an issue about it!

* Since the custom test runner runs as part of `cargo test`, it won't be able to
  find a `3dsx` that hasn't built yet. `cargo-3ds` doesn't build `3dsx` executables until
  _after_ the cargo command it runs internally, so this means that tests can't depend
  on any features of the `3dsx` (like embedded romFS). A workaround for this is to
  simply build the tests as a separate step before running them, after which the
  runner will be able to find the `3dsx`.

* Doctests require a bit of extra setup to work with the runner, since they don't
  use the crate's `#![test_runner]`. To write doctests, add the following to the
  beginning of the doctest (or `fn main()` if the test defines it):

  ```rust
  let _runner = test_runner::GdbRunner::default();
  ```

  The runner must remain in scope for the duration of the test in order for
  the test output to be printed.
