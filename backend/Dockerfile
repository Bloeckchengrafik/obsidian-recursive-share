FROM rust:1.86 AS builder

WORKDIR /build
COPY . .

RUN rustup target add x86_64-unknown-linux-musl
RUN RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine AS runner
WORKDIR /app

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/backend /app/backend

ENTRYPOINT [ "/app/backend" ]
