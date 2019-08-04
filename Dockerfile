FROM ekidd/rust-musl-builder:nightly AS build
COPY . ./
RUN sudo chown -R rust:rust .
RUN cargo build --release

FROM scratch
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-bitcoin-prometheus /
ENV PORT 8000
EXPOSE ${PORT}
CMD ["/rust-bitcoin-prometheus"]