FROM rust:1.63 as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y libssl-dev pkg-config
RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /app/target/release/rust_sqlx_swagger /usr/local/bin

CMD ["rust_api"]
