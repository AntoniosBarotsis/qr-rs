FROM rust:1.64-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /qr-rs
ADD ./server ./server
ADD ./lib ./lib
ADD ./common ./common
ADD ./assets ./assets
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN cargo build --release

FROM scratch
EXPOSE 8080

COPY --from=builder /qr-rs/target/release/qr-rs .
CMD ["./qr-rs"]
