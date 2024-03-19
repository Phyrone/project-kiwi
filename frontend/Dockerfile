FROM oven/bun AS BUILD
WORKDIR /build
ENV NODE_ENV=production
ENV BUILD_TARGET=bun
COPY . .
RUN bun install
RUN bun run build
FROM oven/bun
COPY --from=BUILD /build/dist /app