## Steps

### Bước 1: Khởi tạo Project Node.js

### Bước 2: Cài đặt Dependencies (Vũ khí của Node)

Chúng ta cần những thư viện mạnh nhất để "đấu" với Rust:

- Fastify: Nhanh hơn Express, tiệm cận hiệu năng của Axum.
- Sharp: Thư viện xử lý ảnh viết bằng C++, nhanh nhất hệ sinh thái Node.
- Archiver: Để đóng gói file ZIP trong RAM.
- fastify-multipart: Để nhận file gửi lên từ Frontend.

`npm install fastify @fastify/multipart sharp archiver`

### Bước 3: Viết Code Logic (index.js)

### Bước 4: Viết Dockerfile cho Node.js

### Bước 5: Cấu hình docker-compose.yml để "So găng"

### Bước 6: Cấu hình Nginx để điều hướng (nginx.conf)

### Bước 7: Thực hiện Run và So sánh

1. Chạy hệ thống: docker-compose up --build.
2. Chuẩn bị: Một file ảnh khoảng 5MB.
3. Dùng Bombardier (Công cụ "tra tấn"):

- Test Rust: bombardier -c 20 -n 100 http://localhost/api/rust/
- Test Node: bombardier -c 20 -n 100 http://localhost/api/node/
