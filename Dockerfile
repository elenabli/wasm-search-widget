FROM rust:latest AS builder

RUN cargo install wasm-pack

WORKDIR /app

COPY . .

RUN wasm-pack build --release --target web

FROM nginx:alpine

COPY index.html /usr/share/nginx/html/
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg

EXPOSE 80