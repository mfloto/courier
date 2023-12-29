FROM rust:1.74.1 as builder
WORKDIR /usr/src/courier
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install -y musl-tools
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine
COPY --from=builder /usr/src/courier/target/x86_64-unknown-linux-musl/release/courier /usr/local/bin/courier
CMD ["courier"]