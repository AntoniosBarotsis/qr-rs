# QRG CLI

> QR Generate!

## Installation

Currently, as the crate is not published, you can only install it from source with

```sh
cargo install --path cli
```

## Usage

You can get the list of available commands as follows:

```
$ qrgen -h
A CLI for generating QR Codes with a logo overlay

Usage: qrgen.exe [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>  The text the qr code should contain

Options:
  -d, --destination <DESTINATION>  The filename the QR Code should be saved in [default: out.png]
  -s, --size <SIZE>                The size of the QR Code [default: 600]
  -c, --bg-color <BG_COLOR>        The background color of the QR Code (in hex) [default: FFFFFF]
  -l, --logo <LOGO>                The name of the logo to use in the overlay [default: google]
  -h, --help                       Print help
  -V, --version                    Print version
```

I will try to keep this outdated but you should run it for yourself just in case some command
detail has changed.

### Example Usage

Create a QR Code pointing to `github.com` and dump the image to a file called `tmp.png`.

```sh
qrgen github.com -d tmp.png
```
