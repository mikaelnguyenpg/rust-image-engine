import os
import requests
import zipfile
import io
import pytest

BASE_URL = os.getenv("API_URL", "http://localhost")


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
