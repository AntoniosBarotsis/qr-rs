# QR-RS Server

[![dependency status](https://deps.rs/repo/github/AntoniosBarotsis/qr-rs/status.svg?path=server)](https://deps.rs/repo/github/AntoniosBarotsis/qr-rs?path=server)
[![Docker Image](https://img.shields.io/docker/v/antoniosbarotsis/qr-rs)](https://hub.docker.com/r/antoniosbarotsis/qr-rs)

## Usage

Start the server with `cargo run -r` (in the `server` directory) then run the following to generate
a QR Code that points to `google.com` and save it to `tmp.png`. 

```sh
curl http://127.0.0.1:8080/qr?content=google.com --output tmp.png
./tmp.png
```

The endpoint documentation can be read with

```sh
curl http://127.0.0.1:8080/
```

You can also run this through Docker with

```sh
# docker build -t qr-rs -f server/Dockerfile .
# docker run -p 8080:8080 qr-rs
docker pull antoniosbarotsis/qr-rs
docker run -p 8080:8080 qr-rs
```
