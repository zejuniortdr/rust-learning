# Use a imagem oficial do Rust como base
FROM rust:latest

# Instale as dependências necessárias
WORKDIR /app
COPY . .

# Instale as dependências do projeto e compile
RUN cargo build --release

# Comando de execução
CMD ["cargo", "run", "--release"]
