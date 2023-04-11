FROM rust:1.67

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true
RUN cargo build --release

ENTRYPOINT ["./target/release/zero2prod"]