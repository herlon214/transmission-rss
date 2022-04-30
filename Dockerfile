FROM rust:1.60-stretch as builder
ENV USER root
WORKDIR /usr/src/transmission-rss
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /hello/target/release/transmission-rss /transmission-rss
CMD ["/transmission-rss"]