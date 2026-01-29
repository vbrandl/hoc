FROM blackdex/rust-musl:x86_64-musl-stable as builder

WORKDIR /app
# create new cargo project
RUN cargo init --lib \
        && echo 'fn main() { println!("Hello, world!"); }' >> src/main.rs \
        && echo 'fn foo() { println!("Hello, world!"); }' >> src/lib.rs

# copy build config
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
# HACK: remove build-dependencies so we have at least some caching
RUN head -n $(($(grep -n "\[build-dependencies\]" Cargo.toml | cut -f1 -d:) - 1)) Cargo.toml | sed '/src\/build.rs/d' > \
        Cargo.toml2  && rm Cargo.toml && mv Cargo.toml2 Cargo.toml
# build to cache dependencies
# delete build cache to prevent caching issues later on
RUN cargo build --release \
        && rm -r ./target/x86_64-unknown-linux-musl/release/.fingerprint/hoc-*

# copy original Cargo.toml (HACK)
COPY ./Cargo.toml ./Cargo.toml
# we need our git folder so we can determine the commitref of HEAD
COPY ./.git ./.git
# copy source code
COPY ./static ./static
COPY ./templates ./templates
COPY ./src ./src
# build source code
RUN cargo build --release

FROM alpine:3.23.3

RUN apk --no-cache add --update git \
        && adduser -D hoc
WORKDIR /home/hoc
USER hoc

# once we don't need a git binary anymore, this should be enough
# FROM scratch
# COPY --from=linuxkit/ca-certificates:v0.7 / /

# COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/hoc .
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/hoc .

ENTRYPOINT ["/home/hoc/hoc"]
