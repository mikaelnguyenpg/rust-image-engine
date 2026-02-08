use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json,
};
use std::net::{SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    message: String,
    engine: String,
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // 1. Cấu hình CORS: Cho phép Next.js "nói chuyện" với Rust
    let cors = CorsLayer::new()
        .allow_origin(Any) // Trong thực tế nên giới hạn ở localhost:3000
        .allow_methods(Any);

    // 2. Định nghĩa các tuyến đường (Routes)
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/process", post(process_image))
        .layer(cors);

    // 3. Khởi chạy Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("🚀 Server Rust đã sẵn sàng tại http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler: Kiểm tra trạng thái
async fn health_check() -> impl IntoResponse {
    Json(Status {
        message: "OK".to_string(),
        engine: "Rust Image Engine v1.0".to_string(),
    })
}

// Handler: Xử lý ảnh (Tạm thời chỉ phản hồi test)
async fn process_image() -> impl IntoResponse {
    "Ảnh của ông đang được gửi tới lò luyện Rust..."
}
