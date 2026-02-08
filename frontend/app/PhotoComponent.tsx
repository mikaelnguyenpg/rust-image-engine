"use client";

import { useEffect, useState } from "react";

export default function PhotoComponent() {
  const [status, setStatus] = useState<{
    message: string;
    engine: string;
  } | null>(null);
  const [loading, setLoading] = useState(true);

  // 1. Kiểm tra kết nối với Rust Backend
  const checkBackend = async () => {
    try {
      const res = await fetch("http://localhost:8080/health");
      const data = await res.json();
      setStatus(data);
      console.log(" * Data: ", data);
    } catch (err) {
      console.error("Rust server chưa bật rồi ông giáo ơi!", err);
    } finally {
      setLoading(false);
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
    </div>
  );
}
