FROM rust:latest AS builder

RUN cargo install wasm-pack

WORKDIR /app

COPY . .

RUN wasm-pack build --release --target web

FROM python:3.9-slim

WORKDIR /web

COPY index.html ./
COPY --from=builder /app/pkg ./pkg

EXPOSE 8000

CMD ["python", "-m", "http.server", "8000"]