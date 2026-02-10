# Stage 1: Install dependencies
FROM node:20-slim AS deps
WORKDIR /app
COPY package*.json ./
RUN npm install

# Stage 2: Build code
FROM node:20-slim AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
# Next.js yêu cầu biến này để tối ưu build
ENV NEXT_TELEMETRY_DISABLED 1
RUN npm run build


# Stage 3: Runtime
FROM node:20-slim AS runner
WORKDIR /app
# Tạo user thường để chạy app cho bảo mật (Security Best Practice)
RUN addgroup --system --gid 1001 nodejs
RUN adduser --system --uid 1001 nextjs
# Chỉ copy những file cần thiết từ bản build standalone
COPY --from=builder /app/public ./public
COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

USER nextjs

EXPOSE 3000
ENV PORT 3000
# Chạy bằng server.js (do standalone mode tạo ra) thay vì npm start
CMD ["node", "server.js"]
