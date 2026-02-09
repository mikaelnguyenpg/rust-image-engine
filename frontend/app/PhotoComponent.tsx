"use client";

import { useEffect, useState } from "react";

const API_BASE = process.env.NEXT_PUBLIC_API_URL || "";

export default function PhotoComponent() {
  const [status, setStatus] = useState<{
    message: string;
    engine: string;
  } | null>(null);
  const [loading, setLoading] = useState(true);
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<string>("");
  const [processedImageUrl, setProcessedImageUrl] = useState<string | null>(
    null,
  );
  const [files, setFiles] = useState<FileList | null>(null);

  console.log(`*** API_BASE: ${API_BASE}`);

  // 1. Kiểm tra kết nối với Rust Backend
  const checkBackend = async () => {
    try {
      console.log(`*** I. API: ${API_BASE}/api/health`);

      const res = await fetch(`${API_BASE}/api/health`);
      const data = await res.json();
      setStatus(data);
      console.log(" * I. Data: ", data);
    } catch (err) {
      console.error("Rust server chưa bật rồi ông giáo ơi!", err);
    } finally {
      setLoading(false);
    }
  };

  const uploadImage = async () => {
    if (!file) return;

    const formData = new FormData();
    formData.append("image", file);

    console.log(" * II.1. Uploaded photo: ", file);
    try {
      console.log(`*** II. API: ${API_BASE}/api/process`);

      const res = await fetch(`${API_BASE}/api/process`, {
        method: "POST",
        body: formData,
      });

      // const text = await res.text();
      // setResult(text);

      // Nhận dữ liệu dưới dạng Blob (Binary Large Object)
      const blob = await res.blob();

      // Tạo một URL tạm thời để hiển thị cái Blob này
      const url = URL.createObjectURL(blob);
      setProcessedImageUrl(url);
    } catch (err) {
      setResult("Lỗi gửi ảnh rồi ông giáo ơi!");
    }
  };

  const uploadImages = async () => {
    if (!files) return;

    const formData = new FormData();
    Array.from(files).forEach((f) => {
      formData.append("image", f);
    });

    try {
      console.log(`*** II. API: ${API_BASE}/api/process`);

      const res = await fetch(`${API_BASE}/api/process`, {
        method: "POST",
        body: formData,
      });

      const blob = await res.blob();
      console.log(" * blob: ", blob);
      const url = window.URL.createObjectURL(blob);

      // Tạo thẻ <a> ẩn để trigger download
      const a = document.createElement("a");
      a.href = url;
      a.download = "images_from_rust.zip";
      document.body.appendChild(a);
      a.click();
      a.remove();
    } catch (err) {
      setResult("Lỗi rồi ông giáo ơi!");
    }
  };

  useEffect(() => {
    checkBackend();
  }, []);

  return (
    <div className="flex flex-col items-center justify-center">
      <h1 className="text-4xl font-bold mb-8">Next.js + Rust Bridge</h1>

      <div className="p-6 border border-slate-700 rounded-xl bg-slate-800 shadow-xl">
        {loading ? (
          <p className="animate-pulse">Đang kết nối tới lò luyện Rust...</p>
        ) : status ? (
          <div className="space-y-2">
            <p className="text-green-400 font-mono">✅ Kết nối thành công!</p>
            <p>
              Message: <span className="text-blue-300">{status.message}</span>
            </p>
            <p>
              Engine: <span className="text-orange-300">{status.engine}</span>
            </p>
          </div>
        ) : (
          <p className="text-red-400">❌ Không thể kết nối tới Backend.</p>
        )}
      </div>

      <button
        onClick={checkBackend}
        className="mt-6 px-4 py-2 bg-blue-600 hover:bg-blue-500 rounded-lg transition"
      >
        Re-check Connection
      </button>

      <div className="mt-10 p-6 border border-dashed border-slate-600 rounded-lg">
        <input
          type="file"
          onChange={(e) => setFile(e.target.files?.[0] || null)}
          className="block w-full text-sm text-slate-400 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-600 file:text-white hover:file:bg-blue-500"
        />
        <button
          onClick={uploadImage}
          className="mt-4 w-full py-2 bg-green-600 rounded-lg font-bold"
        >
          Gửi ảnh vào lò luyện Rust
        </button>
        {result && (
          <p className="mt-4 text-center text-yellow-400 font-mono">{result}</p>
        )}
      </div>

      <div className="mt-10 p-6 border border-dashed border-slate-600 rounded-lg">
        <input
          type="file"
          multiple
          onChange={(e) => setFiles(e.target.files)}
          className="block w-full text-sm text-slate-400 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-600 file:text-white hover:file:bg-blue-500"
        />
        <button
          onClick={uploadImages}
          className="mt-4 w-full py-2 bg-green-600 rounded-lg font-bold"
        >
          Gửi nhiều ảnh vào lò luyện Rust
        </button>
        {result && (
          <p className="mt-4 text-center text-yellow-400 font-mono">{result}</p>
        )}
      </div>

      {processedImageUrl && (
        <div className="mt-8">
          <p className="text-center mb-2">Thành quả từ lò luyện Rust:</p>
          <img
            src={processedImageUrl}
            alt="Processed"
            className="max-w-md rounded-lg shadow-2xl border-4 border-green-500"
          />
        </div>
      )}
    </div>
  );
}
