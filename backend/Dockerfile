# Stage 1: Build (Sử dụng cargo-chef để tối ưu thời gian build)
FROM lukemathwalker/cargo-chef:latest-rust-1.85-slim AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - Stage này sẽ được Docker cache nếu Cargo.toml không đổi
RUN cargo chef cook --release --recipe-path recipe.json
# Build code thật
COPY . .
RUN cargo build --release


# Stage 2: Runtime (Image cuối cùng)
FROM debian:bookworm-slim
WORKDIR /app
# Cài đặt SSL để gọi API hoặc ghi file nếu cần
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
# Chỉ copy file chạy duy nhất
COPY --from=builder /app/target/release/backend ./engine

CMD ["./engine"]
