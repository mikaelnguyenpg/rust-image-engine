use axum::{
    Json, Router, extract::Multipart, response::IntoResponse, routing::{get, post}
};
use rayon::iter::IntoParallelIterator;
use zip::write::FileOptions;
use std::{io::Cursor, net::SocketAddr};
use tower_http::cors::{Any, CorsLayer};
use serde::Serialize;
use rayon::prelude::*; // Import Rayon để dùng .par_iter()
use std::io::Write;

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
    // let addr = SocketAddr::from(([127,0,0,1], 8080));
    let addr = SocketAddr::from(([0,0,0,0], 8080));
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
    let mut files_data = Vec::new();

    // 1. Thu thập tất cả các ảnh gửi lên vào một Vector
    while let Some(field) = multipart.next_field().await.unwrap() {
        println!(" * 1. name: {:?} - file_name: {:?}", field.name(), field.file_name());
        // Xử lý ảnh
        if field.name().unwrap() == "image" {
            let name = field.file_name().unwrap_or("image.png").to_string();
            let data = field.bytes().await.unwrap();

            files_data.push((name, data));
        }
    }

    // 2. PHẦN QUAN TRỌNG NHẤT: Xử lý song song bằng Rayon
    // .into_par_iter() sẽ tự động chia các ảnh cho các nhân CPU khác nhau
    let processed_results: Vec<(String, Vec<u8>)> = files_data
        .into_par_iter()
        .map(|(name, data)| {
            let img = image::load_from_memory(&data).unwrap();

            let resized = img.resize(300, 300, image::imageops::FilterType::Lanczos3);

            // PHẦN QUAN TRỌNG THỨ HAI: xử lý trực tiếp trên RAM
            let mut buffer = Cursor::new(Vec::new());
            resized.write_to(&mut buffer, image::ImageFormat::Png).unwrap();
            (name, buffer.into_inner())
        })
        .collect();
    println!(" - Processed photos: {}", processed_results.len());

    // 4. Đóng gói ZIP ngay trong RAM
    let mut zip_buffer = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut zip_buffer);
        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);

        for (name, bytes) in processed_results {
            zip.start_file(format!("processed_{}", name), options).unwrap();
            zip.write_all(&bytes).unwrap();
        }
        zip.finish().unwrap();
    }

    let final_bytes = zip_buffer.into_inner();
    if final_bytes.is_empty() {
        return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Buffer rỗng").into_response();
    }

    // 5. Trả về ZIP
    axum::response::Response::builder()
        .header("Content-Type", "application/zip")
        .header("Content-Disposition", "attachment; filename=\"processed_images.zip\"")
        .header("Content-Length", final_bytes.len().to_string())
        .body(axum::body::Body::from(final_bytes))
        .unwrap()
        .into_response()
}
