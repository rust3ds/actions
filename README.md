# test-runner-3ds
<!-- TODO: better name! -->

A set of tools for running automated Rust tests against Citra (3DS emulator).

## Usage

`./run.sh 3DSX_FILE`

## Goals

* Docker container for manually running tests against Citra
* GitHub Action for running automated tests
* Rust testing framework (custom runner) for use with the 3ds
* (maybe) Acceptance testing framework or glue for one?

## Workflow / Notes

1. Build a test executable (type tbd)
1. `citra-emu` container: bind-mount test executable and choose it
1. `driver` container perform input / output as needed for test, via VNC

    * possible extension: `3dslink -s` to get actual stdout/stderr (return code?)

    * acceptance testing of images, hopefully via screenshot
