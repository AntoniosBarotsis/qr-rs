# Welcome to QR-RS

A simple [server](./server/) and [CLI](./cli/) that generates QR Codes and overlays a logo on top
of them.

It was developed for [GDSC Delft](https://gdsc.community.dev/delft-university-of-technology/).

<p align="center">
  <img src="https://raw.githubusercontent.com/AntoniosBarotsis/qr-rs/master/assets/example.png" alt="Example" width="250">
</p>

The purpose of this is to allow members to easily generate promotional QR Codes to include in
banners/slides without having to worry about creating accounts on third-party sites and free-tier
limits.

The [core library](https://github.com/AntoniosBarotsis/qr-rs/tree/master/lib) is written in Rust
and is a thin wrapper around [an existing](https://github.com/erwanvivien/fast_qr) crate that
does the actual QR Code generation.

That core functionality is currently exposed through a web server and a command line tool, the
latter of which is recommended for personal use as it is just easier to run.

For details on how to run and use the server or the CLI, refer to their respective pages.

The rest of the stuff mentioned on this page is mostly developer information.

## Project Layout

The project is currently split into multiple crates

- [`lib`](https://github.com/AntoniosBarotsis/qr-rs/tree/master/lib): The core logic of the project
- [`server`](https://github.com/AntoniosBarotsis/qr-rs/tree/master/server): A server you can run locally to use the `lib` crate
- [`cli`](https://github.com/AntoniosBarotsis/qr-rs/tree/master/cli): A command line interface to the `lib` crate
- [`common`](https://github.com/AntoniosBarotsis/qr-rs/tree/master/common): Some reusable code for `server` and `cli`

The `lib` and `cli` crates will be published on [`crates.io`](https://crates.io) while the
`server` crate will be published as a Docker image.

The `cli` might be uploaded to other package managers, however, that's something that I don't
want to tackle alone so if you are looking for a place to contribute, start 
[here](https://github.com/AntoniosBarotsis/qr-rs/issues/11) :)

The `common` crate contains very little code and there's no reason to publish it on its own.

## Deployment

The `lib`, `common` and `cli` are now published automatically to [`crates.io`](https://crates.io).
The `server` is published to [DockerHub](https://hub.docker.com/). Binaries are generated for the
`cli` in [Github Releases](https://github.com/AntoniosBarotsis/qr-rs/releases).

Publishing the `cli` on more commonly used package managers
[has been suggested](https://github.com/AntoniosBarotsis/qr-rs/issues/11) but will likely not be
tackled anytime soon (unless you are willing to help!).


## Contributing

Any feature requests, bug reports or general feedback is welcome!

Feel free to [open an issue](https://github.com/AntoniosBarotsis/qr-rs/issues/new/choose) or
[start a discussion](https://github.com/AntoniosBarotsis/qr-rs/discussions/new/choose) in the
repository.

If you want to contribute with code, you can take a look at the
["help wanted" issues](https://github.com/AntoniosBarotsis/qr-rs/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3A%22help+wanted%22) as I will most likely not be implementing those myself.

A *decent* CI pipeline is in place to ensure that everything builds and that all tests pass. I like
being strict when it comes to formatting and linting so there is a CI job for that as well.
