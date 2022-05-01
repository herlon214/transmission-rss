####################################################################################################
## Builder
####################################################################################################
FROM rust:1.60.0 AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev libssl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=transmission-rss
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /app

COPY ./ .

RUN cargo install --target x86_64-unknown-linux-musl --path .


####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Copy our build
COPY --from=builder /usr/local/cargo/bin/transmission-rss .

ENTRYPOINT ["./transmission-rss"]