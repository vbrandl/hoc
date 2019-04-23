FROM ekidd/rust-musl-builder:stable as builder

# create new cargo project
RUN USER=rust cargo init --bin
# copy build config
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# build to cache dependencies
RUN cargo build --release
# delete build cache to prevent caching issues later on
RUN rm -r ./target/x86_64-unknown-linux-musl/release/.fingerprint/hoc-*

# copy source code
COPY ./.git ./.git
COPY ./static ./static
COPY ./templates ./templates
COPY ./src ./src
# build source code
RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add --update git
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/hoc /hoc

ENTRYPOINT ["/hoc"]
