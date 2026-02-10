const fastify = require("fastify")({ logger: false });
const multipart = require("@fastify/multipart");
const sharp = require("sharp");
const archiver = require("archiver");

// Đăng ký plugin xử lý multipart (file upload)
fastify.register(multipart, {
  limits: {
    fileSize: 50 * 1024 * 1024, // Limit input 50Mb
  },
});

fastify.post("/api/process", async (req, reply) => {
  const parts = req.files();

  // 1. Thu thập và xử lý ảnh (Resize)
  const processedImages = [];
  for await (const part of parts) {
    if (part.fieldname === "image") {
      const buffer = await part.toBuffer();
      try {
        const resizedBuffer = await sharp(buffer)
          .resize(300, 300, {
            fit: "cover",
            kernel: "lanczos3",
          })
          .png()
          .toBuffer();
        processedImages.push({ name: part.filename, buffer: resizedBuffer });
      } catch (err) {
        return reply.code(400).send({ error: "Invalid Image format" });
      }
    }
  }

  // const processingPromises = [];
  // for await (const part of parts) {
  //   if (part.fieldname === "image") {
  //     const filename = part.filename;
  //     const promise = part.toBuffer().then((buffer) =>
  //       sharp(buffer)
  //         .resize(300, 300, {
  //           fit: "cover",
  //           kernel: "lanczos3",
  //         })
  //         .png()
  //         .toBuffer()
  //         .then((resizedBuffer) => ({ name: filename, buffer: resizedBuffer })),
  //     );
  //     processingPromises.push(promise);
  //   }
  // }
  // processedImages = await Promise.all(processingPromises);

  // 2. Đóng gói ZIP ngay trong RAM
  const archive = archiver("zip", { store: true });
  const chunks = [];

  archive.on("data", (chunk) => chunks.push(chunk));

  for (const img of processedImages) {
    archive.append(img.buffer, { name: `processed_${img.name}` });
  }

  await archive.finalize();

  const finalZipBuffer = Buffer.concat(chunks);

  // 3. Trả về kết quả
  reply
    .header("Content-Type", "application/zip")
    .header("Content-Disposition", 'attachment; filename="node_processed.zip"')
    .header("Content-Length", finalZipBuffer.length)
    .send(finalZipBuffer);
});

// Chạy tại cổng 8081 để không đụng hàng với Rust (8080)
fastify.listen({ port: 8081, host: "0.0.0.0" }, (err) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
  console.log("🚀 Node.js Server ready at http://0.0.0.0:8081");
});
