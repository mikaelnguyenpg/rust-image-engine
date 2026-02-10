use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post}
};
use rayon::iter::IntoParallelIterator;
use tokio::task;
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
    let api_routes = Router::new()
        .route("/health", get(health_check))
        .route("/process", post(process_images));
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .layer(DefaultBodyLimit::max(5 * 1024 * 1024)); // Limit input size: 50Mb

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

async fn collect_images(mut multipart: Multipart) -> Result<Vec<(String, Bytes)>, StatusCode> {
    let mut files_data = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.file_name().unwrap_or("image.png").to_string();

        match field.bytes().await {
            Ok(data) => files_data.push((name, data)),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    }

    Ok(files_data)
}

fn resize_images(files_data: Vec<(String, Bytes)>) -> Result<Vec<(String, Vec<u8>)>, StatusCode> {
    files_data
        .into_par_iter()
        .map(|(name, data)| -> Result<(String, Vec<u8>), StatusCode> {
            let img = image::load_from_memory(&data)
                .map_err(|_| StatusCode::BAD_REQUEST)?;

            let resized = img.resize(300, 300, image::imageops::FilterType::Lanczos3);
            let mut buffer = Vec::new();
            resized.write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok((name, buffer))
        })
        .collect()
}

fn compress_images(list: Vec<(String, Vec<u8>)>) -> Result<Vec<u8>, StatusCode> {
    let mut zip_buffer: Vec<u8> = Vec::new();
    {
        let mut zip = zip::ZipWriter::new(Cursor::new(&mut zip_buffer));
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for (name, bytes) in list {
            zip.start_file(format!("processed_{}", name), options)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            zip.write_all(&bytes)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        };
        zip.finish()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    Ok(zip_buffer)
}

// Handler: Xử lý ảnh (Tạm thời chỉ phản hồi test)
async fn process_images(multipart: Multipart) -> Response {
    // 1. Thu thập data (Hạn chế unwrap, dùng while let)
    let files_data = match collect_images(multipart).await {
        Ok(data) => data,
        Err(status) => return status.into_response(),
    };

    // 2. Đưa việc nặng sang Rayon một cách an toàn
    // spawn_blocking giúp không làm treo các request khác của Axum
    let processing_result = task::spawn_blocking(move || -> Result<Vec<u8>, StatusCode> {
        // Xử lý song song
        // 3.a. Đồng bộ kích thước ảnh
        let list = resize_images(files_data)?;

        // 3.b. Đóng gói ZIP ngay trong thread này
        compress_images(list)
    }).await;

    // 4. Trả về kết quả (Xử lý lỗi JoinError của Tokio)
    match processing_result {
        Ok(Ok(final_zip)) => {
            Response::builder()
                .header("Content-Type", "application/zip")
                .header("Content-Disposition", "attachment; filename=\"processed_images.zip\"")
                .body(Body::from(final_zip))
                .unwrap()
        },
        Ok(Err(status)) => (status, "Processing error").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Worker thread panicked").into_response(),
    }
}

