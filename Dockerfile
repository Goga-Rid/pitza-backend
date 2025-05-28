# Этап сборки
FROM rust:1.76 as builder
WORKDIR /app

# Копируем файлы зависимостей и устанавливаем их (кэширование)
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -f target/release/deps/pitza-backend*

# Копируем исходники и собираем проект
COPY . .
RUN cargo build --release

# Финальный образ
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Копируем бинарник из builder
COPY --from=builder /app/target/release/pitza-backend .

# Копируем миграции
COPY migrations ./migrations

EXPOSE 8080

CMD ["./pitza-backend"]