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

## To do work

* [ ] Reorganize docker build files vs runtime files a bit
* [ ] Make this repo useable as a github action
* [ ] Run itself as part of CI? I guess?
* [ ] Simpler user-run workflow:
  * Ideally, a single command to spin everything up, build + load a 3dsx and run a vdo script.
  * Maybe cargo args passed in as environment variable or something?
* [ ] Clearly defined dependencies + use cases:
  * Should this be usable without Rust?
  * Is docker the only real dependency?
  * Does this need a separate binary, or can we just use native cargo test
    capabilities?
