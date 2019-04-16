FROM ekidd/rust-musl-builder:stable as builder

# create new cargo project
RUN USER=rust cargo init --bin
# set last modified date to 1970-01-01 to prevent caching issues later on
# RUN touch -t 197001010000.00 src/main.rs
# copy build config
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# build to cache dependencies
RUN cargo build --release
# RUN ls target/x*/release
# RUN touch -t 197001010000.00 ./target/x86_64-unknown-linux-musl/release/hoc.d
RUN rm -r ./target/x86_64-unknown-linux-musl/release/.fingerprint/hoc-*

# copy source code
COPY ./src ./src
# build source code
RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add --update git
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/hoc /hoc
