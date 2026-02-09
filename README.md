# 🚀 Rust-Image-Engine: High-Performance Image Processing Platform

## 🎯 1. Mục tiêu & Tầm nhìn (Vision)

Dự án được xây dựng để giải quyết bài toán xử lý ảnh khối lượng lớn với hiệu suất tối đa.
Mục tiêu cốt lõi là chứng minh sức mạnh của Rust trong việc thay thế các Runtime truyền thống (như Node.js) ở các tác vụ nặng về CPU.

- Ý nghĩa: Cung cấp giải pháp xử lý ảnh an toàn về bộ nhớ, tốc độ Native và khả năng mở rộng (Scale) linh hoạt.
- Định hướng: Phát triển thành một nền tảng Cross-platform (Desktop/Mobile) sử dụng chung một lõi Rust (Shared Core).

## 🛠 2. Tech Stack (Hệ sinh thái công nghệ)

| Thành phần       | Công nghệ sử dụng                    |
| ---------------- | ------------------------------------ |
| Frontend         | Next.js 14, TailwindCSS, TypeScript  |
| Core Engine      | Rust (Axum, Rayon, Image crate)      |
| Benchmark Target | Node.js (Fastify, Sharp)             |
| Infrastructure   | Nginx, Docker, Docker Compose        |
| Testing          | Bombardier (Stress Test), Cargo Test |

## 📂 3. Cấu trúc thư mục (Project Structure)

```bash
├── backend-rust/ # Lõi xử lý ảnh tốc độ cao (Rust)
├── backend-node/ # Backend đối chứng (Node.js)
├── frontend/ # Giao diện người dùng (Next.js)
├── nginx/ # Cấu hình Reverse Proxy & Load Balancing
├── benchmarks/ # Báo cáo so sánh hiệu năng & biểu đồ
├── .github/workflows/ # Kịch bản CI/CD (Tự động build/test)
└── docker-compose.yml # Nhạc trưởng điều phối toàn bộ hệ thống
```

## 🔄 4. Luồng phát triển (Development Workflow)

1. Request: User upload danh sách ảnh từ Frontend.
2. Proxy: Nginx nhận request và điều phối sang Rust Backend qua đường dẫn /api/rust/.
3. Processing: Rust sử dụng Rayon để băm nhỏ dữ liệu, tận dụng tối đa các nhân CPU để resize ảnh song song.
4. Packaging: Kết quả được đóng gói thành file ZIP ngay trong RAM (không ghi đĩa để tăng tốc).
5. Response: Trả về stream dữ liệu cho người dùng.

## 🚀 5. Bắt đầu như thế nào? (Quick Start)

### Yêu cầu hệ thống

- Docker & Docker Compose
- Bombardier (để chạy benchmark)

### Triển khai

#### Triển khai xịn(nginx system)

```bash
# 1. Clone dự án
git clone https://github.com/your-username/rust-image-engine.git

# 2. Khởi động toàn bộ hệ thống (Frontend, 2 Backends, Nginx)
docker-compose up --build -d

# 3. Truy cập giao diện
# FE: http://localhost
# BE: http://localhost/api/health
```

#### Triển khai nhanh(docker-compose system)

```bash
# 1. Clone dự án
git clone https://github.com/your-username/rust-image-engine.git

# 2. Khởi động toàn bộ hệ thống (Frontend, 2 Backends)
docker-compose up --build -d

# 3. Truy cập giao diện
# FE: http://localhost:3000
# BE: http://localhost:8080/api/health
```

#### Triển khai chậm(local system)

```bash
# 1. Clone dự án
git clone https://github.com/your-username/rust-image-engine.git

# 2. Khởi động toàn bộ hệ thống (Frontend, 2 Backends)
# mở 2 terminal:
# - 1 cái cd vào thư mục backend
cd backend && cargo run
# - 1 cái cd vào thư mục frontend
cd frontend && npm run dev

# 3. Truy cập giao diện
# FE: http://localhost:3000
# BE: http://localhost:8080/api/health
```

### Chạy Benchmark so sánh

```bash
# 1. Dọn dẹp các container cũ
docker-compose down

# 2. Build và khởi động (Nhớ bật BuildKit để build Rust nhanh hơn)
DOCKER_BUILDKIT=1 docker-compose up --build -d

# 3. Theo dõi log của cả 2 phe khi đang benchmark
docker-compose logs -f backend node-backend
```

```bash
# Test hiệu năng Rust
# bombardier -c 50 -d 30s -m POST -f test.jpg http://localhost/api/rust/process
# Modify to `/api/rust` in script.js
docker run --rm --add-host=host.docker.internal:host-gateway -v $(pwd):/home/k6 -i grafana/k6 run /home/k6/script.js

# Test hiệu năng Node.js
# bombardier -c 50 -d 30s -m POST -f test.jpg http://localhost/api/node/process
# Modify to `/api/node` in script.js
docker run --rm --add-host=host.docker.internal:host-gateway -v $(pwd):/home/k6 -i grafana/k6 run /home/k6/script.js
```

## 📈 6. Kết quả Benchmark (Performance Results)

> "Rust xử lý nhanh hơn Node.js 2.5x trong điều kiện 100 requests đồng thời,
> mức chiếm dụng RAM thấp hơn 4x."
