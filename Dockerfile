FROM rust:1.83 AS wasm
WORKDIR /app
RUN cargo install wasm-pack
COPY libtetris/ /app/libtetris/
COPY web-wasm/ /app/web-wasm/
COPY tree-bot/ /app/tree-bot/
COPY pc-finder/ /app/pc-finder
RUN wasm-pack build web-wasm

FROM node:lts AS frontend
ARG BASE_URL /
WORKDIR /app/web-ui
COPY web-ui/package*.json /app/web-ui/
COPY --from=wasm /app/web-wasm /app/web-wasm
RUN npm ci
COPY web-ui/ /app/web-ui/
RUN npm run build

FROM thesilican/httpd
COPY --from=frontend /app/web-ui/dist/ /public/
