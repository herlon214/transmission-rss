FROM rust:1.60.0 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/transmission-rss
COPY . .
RUN cargo build --release

RUN cargo install --target x86_64-unknown-linux-musl --path .

# Copy the statically-linked binary into a scratch container.
FROM scratch
COPY --from=build /usr/local/cargo/bin/transmission-rss .
USER 1000
CMD ["./transmission-rss"]