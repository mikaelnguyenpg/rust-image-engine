use axum::{
    Json, Router, extract::Multipart, response::IntoResponse, routing::{get, post}
};
use image::load_from_memory;
use std::{io::Cursor, net::SocketAddr};
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
async fn process_image(mut multipart: Multipart) -> impl IntoResponse {
    // "Ảnh của ông đang được gửi tới lò luyện Rust..."
    let mut file_name = String::new();
    let mut file_size = 0;
    let mut processed_bytes = Vec::new();

    // Duyệt qua các "trường" (fields) trong form data gửi lên
    while let Some(field) = multipart.next_field().await.unwrap() {
        // file_name = field.file_name().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        // file_size = data.len();

        // println!("Nhận được file: {} với dung lượng: {} bytes", file_name, file_size);

        // Xử lý ảnh
        if field.name().unwrap() == "image" {
            let data = field.bytes().await.unwrap();

            // 1. Load ảnh từ mảng byte trong RAM
            let img = load_from_memory(&data).expect("Không đọc được định dạng ảnh");

            // 2. Xử lý: Biến thành ảnh trắng đen (Grayscale)
            // Rust xử lý việc này cực nhanh vì nó tối ưu ở mức CPU
            let processed_img = img.grayscale();

            // 3. Ghi dữ liệu đã xử lý vào một "buffer" (vùng đệm) trong RAM
            let mut buffer = Cursor::new(Vec::new());
            processed_img.write_to(&mut buffer, image::ImageFormat::Png).expect("Lỗi khi ghi ảnh");

            processed_bytes = buffer.into_inner();
        }
    }

    // format!("Rust đã nhận: {} ({} bytes). Quá nhẹ nhàng!", file_name, file_size)

    // 4. Trả về mảng byte ảnh trực tiếp cho Frontend
    // Chúng ta thêm Header để trình duyệt hiểu đây là ảnh PNG
    axum::response::Response::builder().header("Content-Type", "image/png").body(axum::body::Body::from(processed_bytes)).unwrap()
}
