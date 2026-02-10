import io
import os
import pytest
import requests
import zipfile

BASE_URL = os.getenv("API_URL", "http://localhost")


# --- Test Case 1: Thông luồng FE-BE ---
@pytest.mark.parametrize("endpoint", ["/api/rust/process", "/api/node/process"])
def test_image_procesing_flow(endpoint):
    # 1. Chuẩn bị ảnh test
    url = f"{BASE_URL}{endpoint}"
    files = {"image": ("test.jpg", open("test-photo.jpg", "rb"), "image/jpeg")}

    # 2. Gửi request
    response = requests.post(url, files=files)

    # 3. Kiểm tra HTTP Status
    assert response.status_code == 200
    assert response.headers["Content-Type"] == "application/zip"

    # 4. Kiểm tra nội dung ZIP
    with zipfile.ZipFile(io.BytesIO(response.content)) as z:
        # Kiểm tra xem có file bên trong không
        file_list = z.namelist()
        assert len(file_list) > 0

        # Kiểm tra tên file có đúng format prefix không
        assert any("processed_" in name for name in file_list)

        # (Tùy chọn) Kiểm tra xem file ảnh bên trong có hợp lệ không
        with z.open(file_list[0]) as img_file:
            img_data = img_file.read()
            assert len(img_data) > 0


# --- Test Case 2: File quá lớn (>50MB) ---
@pytest.mark.parametrize("endpoint", ["/api/rust/process", "/api/node/process"])
def test_file_too_large(endpoint):
    url = f"{BASE_URL}{endpoint}"
    # Tạo một file giả lập 51MB bằng cách ghi byte trống
    large_file = io.BytesIO(b"0" * (51 * 1024 * 1024))
    files = {"image": ("large.jpg", large_file, "image/jpeg")}

    response = requests.post(url, files=files)

    # Kỳ vọng 413 Payload Too Large
    assert response.status_code == 413


# --- Test Case 3: Gửi file không phải ảnh (.txt) ---
@pytest.mark.parametrize("endpoint", ["/api/rust/process", "/api/node/process"])
def test_invalid_file_type(endpoint):
    url = f"{BASE_URL}{endpoint}"
    # Tạo một file giả lập 51MB bằng cách ghi byte trống
    files = {"image": ("hacker.txt", b"day khong phai la anh", "text/plain")}

    response = requests.post(url, files=files)

    # Backend nên xử lý lỗi (ví dụ 400 Bad Request) thay vì sập (500)
    assert response.status_code in [400, 422, 415]


# --- Test Case 4: Gửi nhiều ảnh cùng lúc ---
@pytest.mark.parametrize("endpoint", ["/api/rust/process", "/api/node/process"])
def test_multiple_files(endpoint):
    url = f"{BASE_URL}{endpoint}"
    # Chuẩn bị danh sách 3 ảnh (để test nhanh, ông có thể tăng lên 10)
    multiple_files = [
        ("image", (f"photo{x}.jpg", open("test-photo.jpg", "rb"), "image/jpeg"))
        for x in range(3)
    ]

    response = requests.post(url, files=multiple_files)

    assert response.status_code == 200
    with zipfile.ZipFile(io.BytesIO(response.content)) as z:
        # Kiểm tra số lượng file trong ZIP phải khớp số lượng gửi lên
        assert len(z.namelist()) == 3
