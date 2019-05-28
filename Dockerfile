FROM ekidd/rust-musl-builder:stable as builder

# create new cargo project
RUN USER=rust cargo init --bin
# copy build config
COPY --chown=rust ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# HACK: remove build-dependencies so we have at least some caching
RUN head -n $(($(grep -n "\[build-dependencies\]" Cargo.toml | cut -f1 -d:) - 1)) Cargo.toml | sed '/build.rs/d' > \
        Cargo.toml2  && rm Cargo.toml && mv Cargo.toml2 Cargo.toml
# build to cache dependencies
RUN cargo build --release
# delete build cache to prevent caching issues later on
RUN rm -r ./target/x86_64-unknown-linux-musl/release/.fingerprint/hoc-*

# copy original Cargo.toml (HACK)
COPY ./Cargo.toml ./Cargo.toml
# we need our git folder so we can determine the commitref of HEAD
COPY ./.git ./.git
# copy source code
COPY ./static ./static
COPY ./templates ./templates
COPY ./build.rs ./build.rs
COPY ./src ./src
# build source code
RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add --update git

RUN adduser -D hoc
WORKDIR /home/hoc
USER hoc

# once we don't need a git binary anymore, this should be enough
# FROM scratch
# COPY --from=linuxkit/ca-certificates:v0.7 / /

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/hoc .

ENTRYPOINT ["/home/hoc/hoc"]
