# Lưu ý: Ông phải mở file /frontend/next.config.js và thêm dòng: output: 'standalone' thì Dockerfile này mới chạy được.
# Stage 1: Build
FROM node:20-slim AS builder
WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .

# Stage 2: Runtime
FROM node:20-slim
WORKDIR /app
# Chỉ cài các thư viện production
COPY package*.json ./
RUN npm install --omit=dev && npm cache clean --force
# Copy source code
# COPY --from=builder /app/package.json ./package.json
# COPY --from=builder /app/node_modules ./node_modules
COPY --from=builder /app/index.js ./index.js
# Cài thư viện hệ thống cho Sharp: Cách này tối ưu size hơn Copy-source
RUN apt-get update && apt-get install -y --no-install-recommends && rm -rf /var/lib/apt/lists/*

EXPOSE 80
CMD ["node", "index.js"]
