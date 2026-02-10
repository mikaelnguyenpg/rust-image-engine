use axum::{
    Json, Router,
    body::{Body, Bytes},
    extract::{DefaultBodyLimit, Multipart},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::*; // Import Rayon để dùng .par_iter()
use serde::Serialize;
use std::io::Write;
use std::{io::Cursor, net::SocketAddr};
use tokio::task;
use tower_http::cors::{Any, CorsLayer};
use zip::write::FileOptions;

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
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
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
            let img = image::load_from_memory(&data).map_err(|_| StatusCode::BAD_REQUEST)?;

            let resized = img.resize(300, 300, image::imageops::FilterType::Lanczos3);
            let mut buffer = Vec::new();
            resized
                .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
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
        }
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
    })
    .await;

    // 4. Trả về kết quả (Xử lý lỗi JoinError của Tokio)
    match processing_result {
        Ok(Ok(final_zip)) => Response::builder()
            .header("Content-Type", "application/zip")
            .header(
                "Content-Disposition",
                "attachment; filename=\"processed_images.zip\"",
            )
            .body(Body::from(final_zip))
            .unwrap(),
        Ok(Err(status)) => (status, "Processing error").into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Worker thread panicked").into_response(),
    }
}

#[cfg(test)] // Chỉ biên dịch khi chạy test
mod tests {
    use zip::ZipArchive;

    use super::*; // Lấy các hàm ở trên xuống để dùng

    #[test]
    fn test_resize_images_logic() {
        // Tạo 1 ảnh 1x1 pixel màu đỏ cực nhỏ để test cho nhanh
        let mut img_buffer = Vec::new();
        let test_img = image::RgbImage::new(1, 1);
        image::DynamicImage::ImageRgb8(test_img)
            .write_to(
                &mut std::io::Cursor::new(&mut img_buffer),
                image::ImageFormat::Png,
            )
            .unwrap();

        let input = vec![("test.png".to_string(), Bytes::from(img_buffer))];

        // Chạy hàm resize (vì hàm sync nên gọi thẳng)
        let result = resize_images(input).expect("Resize failed");

        assert_eq!(result.len(), 1); // Fail nếu độ dài ảnh đọc được không phải =1
        assert_eq!(result[0].0, "test.png"); // Fail nếu tên ảnh thứ nhất trong mảng không phải "test.png"

        // Kiểm tra xem ảnh mới có đúng size không bằng cách load lại
        let output_img = image::load_from_memory(&result[0].1).unwrap();
        assert_eq!(output_img.width(), 300); // Fail nếu width không =300
        assert_eq!(output_img.height(), 300); // Fail nếu height không =300
    }

    #[test]
    fn test_resize_images_logic_edge() {
        use image::{DynamicImage, ImageBuffer, Rgb};
        use std::io::Cursor;

        // --- 1. CHUẨN BỊ: Tạo 1 tấm ảnh 10x10 pixel màu đỏ "xịn" ---
        let mut img_data = Vec::new();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(10, 10);
        DynamicImage::ImageRgb8(img)
            .write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Png)
            .expect("Tạo ảnh giả thất bại");

        let input = vec![("test_image.png".to_string(), Bytes::from(img_data))];

        // --- 2. THỰC THI ---
        let result = resize_images(input).expect("Hàm resize bị lỗi");

        // --- 3. KIỂM CHỨNG ---
        assert_eq!(result.len(), 1);
        let (name, output_bytes) = &result[0];
        assert_eq!(name, "test_image.png");

        // Load lại ảnh đầu ra để kiểm tra kích thước
        let output_img =
            image::load_from_memory(output_bytes).expect("Dữ liệu đầu ra không phải là ảnh hợp lệ");

        // Theo logic hàm của ông, nó phải là 300x300
        assert_eq!(output_img.width(), 300);
        assert_eq!(output_img.height(), 300);
    }

    #[test]
    fn test_compress_images_logic_happy_v1() {
        let list = vec![
            ("a.png".to_string(), vec![1, 2, 3]),
            ("b.png".to_string(), vec![2, 3, 4]),
        ];

        let zip_data = compress_images(list).expect("Compression failed");

        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(zip_data)).unwrap();

        assert_eq!(archive.len(), 2); // Fail nếu giải nén ra không có đủ 2 ảnh
        // tên của 2 file giải nén ra phải đúng
        assert!(archive.by_name("processed_a.png").is_ok()); // Fail nếu giải nén ra không có file đúng name "processed_a.png"
        assert!(archive.by_name("processed_b.png").is_ok()); // Fail nếu giải nén ra không có file đúng name "processed_b.png"
    }

    #[test]
    fn test_compress_images_logic_happy_v2() {
        // --- 1. CHUẨN BỊ (ARRANGE) ---
        // Giả lập danh sách ảnh đã xử lý
        let list = vec![
            ("image1.png".to_string(), b"data_of_image_1".to_vec()),
            ("photo2.png".to_string(), b"data_of_image_2".to_vec()),
        ];

        // --- 2. THỰC THI (ACT) ---
        let result = compress_images(list);

        // --- 3. KIỂM CHỨNG (ASSERT) ---
        // Kiểm tra xem hàm có trả về Ok không
        assert!(result.is_ok(), "Hàm compress_images phải trả về Ok");
        let zip_data = result.unwrap();

        // Dùng ZipArchive để "mổ xẻ" cái file ZIP vừa tạo ra
        let mut archive = ZipArchive::new(Cursor::new(zip_data))
            .expect("Dữ liệu trả về phải là một file ZIP hợp lệ");

        // Kiểm tra số lượng file bên trong
        assert_eq!(archive.len(), 2, "File ZIP phải chứa đúng 2 file");

        {
            // Kiểm tra tên file và nội dung file thứ nhất
            let mut file1 = archive
                .by_name("processed_image1.png")
                .expect("Không tìm thấy file 1");
            assert_eq!(file1.name(), "processed_image1.png");

            // (Tùy chọn) Kiểm tra xem dữ liệu có bị sai lệch không
            let mut content1 = Vec::new();
            std::io::copy(&mut file1, &mut content1).unwrap();
            assert_eq!(content1, b"data_of_image_1");
        }

        {
            // Kiểm tra tên file thứ hai
            let file2 = archive
                .by_name("processed_photo2.png")
                .expect("Không tìm thấy file 2");
            assert_eq!(file2.name(), "processed_photo2.png");
        }
    }

    #[test]
    fn test_compress_images_edge_empty_list() {
        let list: Vec<(String, Vec<u8>)> = vec![];

        let result = compress_images(list);

        assert!(result.is_ok(), "Hàm phải xử lý được danh sách rỗng");
        let zip_data = result.unwrap();

        // Kiểm tra xem file ZIP tạo ra có hợp lệ không
        let archive =
            ZipArchive::new(Cursor::new(zip_data)).expect("ZIP rỗng vẫn phải đúng định dạng");

        assert_eq!(archive.len(), 0, "File ZIP phải có 0 phần tử");
    }

    #[test]
    fn test_compress_images_special_names() {
        let list = vec![
            ("ảnh đẹp.png".to_string(), b"data1".to_vec()),
            ("sub/folder/file.jpg".to_string(), b"data2".to_vec()),
            ("../danger.txt".to_string(), b"data3".to_vec()),
        ];

        let result = compress_images(list).expect("Nén file có tên đặc biệt thất bại");
        let mut archive = ZipArchive::new(Cursor::new(result)).unwrap();

        // Kiểm tra xem ZipWriter có giữ nguyên các ký tự đặc biệt không
        {
            let file1 = archive
                .by_name("processed_ảnh đẹp.png")
                .expect("Lỗi ký tự Unicode");
            assert_eq!(file1.name(), "processed_ảnh đẹp.png");
        }

        {
            let file2 = archive
                .by_name("processed_sub/folder/file.jpg")
                .expect("Lỗi ký tự gạch chéo");
            assert_eq!(file2.name(), "processed_sub/folder/file.jpg");
        }
    }
}
