# === Этап сборки ===
FROM rust:1.82 as builder
WORKDIR /app

# Копируем файлы зависимостей и устанавливаем их (кэширование)
COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release || true
RUN rm -f target/release/deps/pitza_backend*

# Копируем исходники и собираем проект
COPY . .
RUN cargo build --release

# Устанавливаем diesel_cli для использования в финальном образе
RUN cargo install diesel_cli --no-default-features --features postgres

# === Финальный образ ===
FROM debian:bookworm-slim
RUN apt-get update && \
    apt-get install -y libpq-dev ca-certificates gcc pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем бинарник из builder
COPY --from=builder /app/target/release/pitza_backend .

# Копируем diesel CLI
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Копируем миграции
COPY migrations ./migrations

EXPOSE 8080

CMD ["sh", "-c", "cd /app && diesel migration run && ./pitza_backend"]