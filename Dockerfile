FROM rust:1.57 AS build

WORKDIR /usr/src/app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM scratch

COPY --from=build /usr/src/app/target/x86_64-unknown-linux-musl/release/app /app

CMD ["/app"]
