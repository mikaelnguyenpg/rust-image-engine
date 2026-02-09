# 🚀 Rust-Image-Engine: High-Performance Image Processing Platform

![Rust](https://img.shields.io/badge/backend-Rust-orange?style=for-the-badge&logo=rust)
![Node.js](https://img.shields.io/badge/backend-Node.js-green?style=for-the-badge&logo=node.js)
![Nextjs](https://img.shields.io/badge/frontend-Nextjs-black?style=for-the-badge&logo=typescript)
![Python](https://img.shields.io/badge/test-Python-blue?style=for-the-badge&logo=python)
![Docker](https://img.shields.io/badge/infra-Docker-blue?style=for-the-badge&logo=docker)
![K6](https://img.shields.io/badge/load-Grafana/K6-blueviolet?style=for-the-badge&logo=k6)
![Status](https://img.shields.io/badge/tests-passing-brightgreen?style=for-the-badge)

## 🎯 1. Mục tiêu & Tầm nhìn (Vision)

Dự án được xây dựng để giải quyết bài toán xử lý ảnh khối lượng lớn với hiệu suất tối đa.
Mục tiêu cốt lõi là chứng minh sức mạnh của Rust trong việc thay thế các Runtime truyền thống (như Node.js) ở các tác vụ nặng về CPU.

- Ý nghĩa: Cung cấp giải pháp xử lý ảnh an toàn về bộ nhớ, tốc độ Native và khả năng mở rộng (Scale) linh hoạt.
- Định hướng: Phát triển thành một nền tảng Cross-platform (Desktop/Mobile) sử dụng chung một lõi Rust (Shared Core).

## 🏗️ 2. Kiến trúc hệ thống

```mermaid
graph LR
    A[Client Request] --> B{Nginx Proxy}
    B -- /api/rust --> C[Rust Engine]
    B -- /api/node --> D[Node.js Engine]

    subgraph "Rust: Parallel Processing"
    C --> C1[Rayon Worker 1]
    C --> C2[Rayon Worker 2]
    end

    subgraph "Node.js: Event-driven"
    D --> D1[Sharp/libvips]
    end

    C1 & C2 & D1 --> E[Stream ZIP Result]
    E --> A
```

## 📊 3. Benchmark Results

Thực hiện bài test bằng k6 với cấu hình: 20 VUs, 30s duration.
Cả hai container được giới hạn tài nguyên tại 0.5 CPU và 256MB RAM.

| Metric                | Rust (Axum)      | Node.js (Fastify) | Winner         |
| --------------------- | ---------------- | ----------------- | -------------- |
| Throughput (Reqs/sec) | 5.90             | 3.84              | 🚀 Rust (+53%) |
| Avg Latency           | 3.22s            | 4.91s             | ⚡ Rust        |
| P(95) Latency         | 4.49s            | 5.99s             | 🛡️ Rust        |
| Memory Stability      | Extremely Stable | Fluctuating (GC)  | 💎 Rust        |

## 🛠 4. Tech Stack (Hệ sinh thái công nghệ)

| Thành phần       | Công nghệ sử dụng                       |
| ---------------- | --------------------------------------- |
| Frontend         | Next.js 14, TailwindCSS, TypeScript     |
| Core Engine      | Rust (Axum, Rayon, Image crate)         |
| Benchmark Target | Node.js (Fastify, Sharp, Archiver)      |
| Infrastructure   | Nginx, Docker, Docker Compose           |
| Testing          | Pytest(Integration test), k6(Load test) |

## 📂 5. Cấu trúc thư mục (Project Structure)

```bash
├── backend-rust/ # Lõi xử lý ảnh tốc độ cao (Rust)
├── backend-node/ # Backend đối chứng (Node.js)
├── frontend/ # Giao diện người dùng (Next.js)
├── nginx/ # Cấu hình Reverse Proxy & Load Balancing
├── benchmarks/ # Báo cáo so sánh hiệu năng & biểu đồ
├── .github/workflows/ # Kịch bản CI/CD (Tự động build/test)
└── docker-compose.yml # Nhạc trưởng điều phối toàn bộ hệ thống
```

## 🔄 6. Luồng phát triển (Development Workflow)

1. Request: User upload danh sách ảnh từ Frontend.
2. Proxy: Nginx nhận request và điều phối sang Rust Backend qua đường dẫn /api/rust/.
3. Processing: Rust sử dụng Rayon để băm nhỏ dữ liệu, tận dụng tối đa các nhân CPU để resize ảnh song song.
4. Packaging: Kết quả được đóng gói thành file ZIP ngay trong RAM (không ghi đĩa để tăng tốc).
5. Response: Trả về stream dữ liệu cho người dùng.

## 🚀 7. Bắt đầu như thế nào? (Quick Start)

### 1. Requirements

- Docker & Docker Compose.
- Một file ảnh `test-photo.jpg` ở thư mục gốc.

### 2. Up and Running

Sử dụng Docker BuildKit để tối ưu hóa thời gian build:

```Bash
DOCKER_BUILDKIT=1 docker-compose up --build
```

### 3. Manual Test

- trên Browser, truy cập `http://localhost/api/health` để check BE API
- truy cập `http://localhost` để check FE Nextjs
  - click `Choose files` để chọn nhiều file ảnh
  - click `Gửi nhiều ảnh...` để gửi ảnh lên BE Service. BE sẽ nén từng ảnh và nén tệp ảnh thành 1 file ZIP. Rồi FE Nextjs down file ZIP đó về.

### 4. Running Integration Tests

Kiểm tra tính đúng đắn của luồng dữ liệu thông qua container Tester:

```Bash
docker-compose run --rm tester
```

### 5. Running Load Test (k6)

```Bash
# 1. Modify to `/api/rust/process` or `/api/node/process` in script.js
# 2.(a) Linux, Mac
docker run --rm --add-host=host.docker.internal:host-gateway -v $(pwd):/home/k6 -i grafana/k6 run /home/k6/script.js
# 2.(b) Windows
```

### 6. (Optional) Manually test local

```bash
# 1 terminal run BE Rust
cd backend && cargo run
# 1 terminal run FE Nextjs
cd frontend && npm run dev

# Manual test: access http://localhost:8080/api/health to check BE
# Manual test: access http://localhost:3000/ to check FE

# 1 terminal Integration-Test: install uv manager for Python
uv sync
uv run pytest tests
```

## 📈 8. Tổng kết

> "Rust xử lý nhanh hơn Node.js 1.5x trong điều kiện 20 requests đồng thời,
> mức chiếm dụng RAM thấp hơn 4x."

```plaintext
Performance Comparison (Throughput - Reqs/sec)
--------------------------------------------
Rust:   ███████████████████████████ 5.90 reqs/s
Node:   ██████████████ 3.84 reqs/s
--------------------------------------------
```

![Performance Comparison](<https://quickchart.io/chart?c={type:%27bar%27,data:{labels:[%27Throughput%20(req/s)%27,%27Avg%20Latency%20(s)%27],datasets:[{label:%27Rust%27,data:[5.9,3.22]},{label:%27Node.js%27,data:[3.84,4.91]}]}}>)

## 🧠 Key Learnings

**Memory Management**: Rust không có Garbage Collector giúp tài nguyên RAM luôn ổn định dưới tải cao, trong khi Node.js dễ bị nghẽn (bottleneck) khi GC hoạt động để dọn dẹp các Buffer ảnh lớn.

**Parallelism**: Nhờ Rayon, Rust tận dụng tối đa 0.5 CPU để xử lý tính toán song song hiệu quả hơn cơ chế Event Loop kết hợp worker pool của Node.js.

**Scalability**: Kiến trúc sử dụng Nginx làm Reverse Proxy giúp việc mở rộng (Horizontal Scaling) các instance Backend trở nên dễ dàng.

Created with ❤️ by Gemini & [Mikael]
