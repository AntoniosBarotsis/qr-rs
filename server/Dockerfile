FROM rust:1.64-alpine as builder

RUN apk add --no-cache musl-dev libressl-dev

ENV CARGO_INCREMENTAL=0

WORKDIR /qr-rs
ADD ./server ./server
ADD ./lib ./lib
ADD ./common ./common
ADD ./assets ./assets
WORKDIR /qr-rs/server
RUN cargo build --release

FROM scratch
EXPOSE 8080

COPY --from=builder /qr-rs/server/target/release/qr-rs .
CMD ["./qr-rs"]
