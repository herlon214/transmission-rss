FROM ekidd/rust-musl-builder
WORKDIR /usr/src
COPY . .
ADD --chown=rust:rust . ./

CMD cargo build --release

FROM scratch
COPY --from=builder /usr/src/target/release/transmission-rss /transmission-rss
CMD ["/transmission-rss"]