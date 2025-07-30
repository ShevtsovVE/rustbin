FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libssl-dev
COPY --from=builder /app/target/release/rustbin /usr/local/bin/
WORKDIR /data
CMD ["rustbin"]