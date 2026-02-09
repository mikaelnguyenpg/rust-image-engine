import http from "k6/http";
import { check, sleep } from "k6";

// Đọc file ảnh vào bộ nhớ (k6 sẽ dùng file này để upload)
const binFile = open("./test-photo.jpg", "b");

export const options = {
  vus: 20, // Giả lập 20 người dùng cùng lúc (tương đương -c 20 của bombardier)
  duration: "30s", // Chạy trong 30 giây
};

export default function () {
  // Cấu hình Multipart Form Data
  const data = {
    image: http.file(binFile, "test-photo.jpg", "image/jpeg"),
  };

  // 1. NOTE: CHỌN ĐÍCH ĐẾN (Ông đổi 'rust' <----> 'node' khi muốn test phe kia)
  const url = "http://host.docker.internal/api/node/process";
  const res = http.post(url, data);

  // 2. Kiểm tra xem có đúng là trả về mã 200 (Thành công) không
  check(res, {
    "is status 200": (r) => r.status === 200,
  });

  res.sleep(0.1);
}
