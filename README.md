# actions

A set of Github Actions for working with 3DS applications and the `rust3ds` toolchain.

It's recommended to use the `test-runner` crate from [ctru-rs](https://github.com/rust3ds/ctru-rs)
when working with these actions, in order to get useful test output on failures.

## Components

* `setup`: action for setting up the Rust 3DS toolchain in workflows
* `run-tests`: action for running test executables with Citra in workflows

## Usage

This example shows the default value for each of the inputs.

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
        uses: rust3ds/actions/setup@v1
        with:
          # Optionally use a more specific nightly toolchain here if desired
          toolchain: nightly

      - name: Build and run tests
        uses: rust3ds/actions/run-tests@v1
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

* Since the custom test runner runs as part of `cargo test`, it won't be able to
  find a `3dsx` that hasn't built yet. `cargo-3ds` doesn't build `3dsx` executables until
  _after_ the cargo command it runs internally, so this means that tests can't depend
  on any features of the `3dsx` (like embedded romFS). A workaround for this is to
  simply build the tests as a separate step before running them, after which the
  runner will be able to find the `3dsx`.
