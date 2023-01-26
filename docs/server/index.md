# Server

## Usage

Start the server with `cargo run -r` (in the `server` directory) then run the following to generate
a QR Code that points to `google.com` and save it to `tmp.png`. 

```
curl http://127.0.0.1:8080/qr?content=google.com --output tmp.png
./tmp.png
```

You can also run this through Docker with

```
docker build -t qr-rs -f server/Dockerfile .
docker run -p 8080:8080 qr-rs
```

The endpoint documentation can be read with

```
curl http://127.0.0.1:8080/help
```
