FROM oven/bun:debian AS BUILD
WORKDIR /work/frontend/
ENV NODE_ENV=production
ENV BUILD_TARGET=bun
COPY . /work
RUN bun install
RUN bun run build

FROM oven/bun:alpine
COPY --from=BUILD /work/frontend/dist /usr/lib/webapp/
RUN cd /usr/lib/webapp/ && bun install

WORKDIR /app/
CMD bun /usr/lib/webapp/index.js