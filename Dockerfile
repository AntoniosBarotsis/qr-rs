FROM rust:1.64-alpine as builder

RUN apk add --no-cache musl-dev

WORKDIR /opt
RUN cargo new --bin qr-rs
WORKDIR /opt/qr-rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
ADD ./benches ./benches
RUN cargo build --release

RUN rm ./src/*.rs
RUN rm ./target/release/qr-rs

ADD ./src ./src
COPY ./assets/logo.png ./assets/logo.png
RUN cargo build --release

FROM scratch
EXPOSE 8080

WORKDIR /opt/qr-rs
COPY --from=builder /opt/qr-rs/target/release/qr-rs .
CMD ["./qr-rs"]
