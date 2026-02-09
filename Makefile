# Chạy toàn bộ hệ thống qua Nginx (Môi trường chuẩn)
up-nginx:
	docker-compose up --build

# Chạy Docker nhưng mở cổng trực tiếp (Để debug lẻ)
up-debug:
	NEXT_PUBLIC_API_URL=http://localhost:8080 docker-compose up --build

# Dừng hệ thống
down:
	docker-compose down

# Chạy Backend Rust lẻ
run-rust:
	cd backend && cargo run

# Chạy Frontend Nextjs lẻ
run-next:
	cd frontend && npm run dev
