# CLI

## Installation

!!! note
    Make sure you have 
    [Cargo installed](https://doc.rust-lang.org/cargo/getting-started/installation.html)!

```sh
cargo install qrg
```

Alternatively, you can download pre-built binaries from
[GitHub Releases](https://github.com/AntoniosBarotsis/qr-rs/releases).

## Usage

You can get the list of available commands as follows:

```
$ qrg -h
Generates QRCodes with a logo overlay.

Usage: qrg.exe [OPTIONS] <CONTENT>

Arguments:
  <CONTENT>  The text the qr code should contain

Options:
  -d, --destination <DESTINATION>
          The filename the QR Code should be saved in [default: out.png]
  -s, --size <SIZE>
          The size of the QR Code [default: 600]
  -l, --logo <LOGO>
          The name of the logo to use in the overlay. Currently, only the Google logo can be used [default: google]
      --logo-source <LOGO_SOURCE>
          Path to the logo (must be a valid PNG/JPEG) [aliases: path]
      --logo-web-source <LOGO_WEB_SOURCE>
          URL to the logo (must be a valid PNG/JPEG) [aliases: web]
  -c, --bg-color <BG_COLOR>
          The background color of the QR Code (in hex) [default: FFFFFF]
      --logo-bg-color <LOGO_BG_COLOR>
          The background color of the logo (in hex) [default: FFFFFF] [aliases: lc]
  -h, --help
          Print help
  -V, --version
          Print version
```

I will try to keep this updated but you should run it for yourself just in case some command
detail has changed.

### Example Usage

Create a QR Code pointing to `github.com` and dump the image to a file called `tmp.png`.

```sh
qrg github.com -d tmp.png
```

Create a QR Code with the text `Hello, World!` using Github's logo and a dark background

```sh
qrg "Hello, World!" -c 36393e --web https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png
```
