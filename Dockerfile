####################################################################################################
## Builder
####################################################################################################
FROM rust:1.60.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release
RUN strip -s /app/target/x86_64-unknown-linux-musl/release/transmission-rss


####################################################################################################
## Final image
####################################################################################################
FROM alpine

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/transmission-rss ./

ENTRYPOINT ["/app/transmission-rss"]