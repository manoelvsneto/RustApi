# Etapa 1: Construir a aplicação
FROM rust:1.61.0 as builder

WORKDIR /app

# Copiar arquivos de dependências e construir o cache
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
RUN rm src/*.rs

# Copiar o código-fonte restante
COPY . .

# Compilar o projeto
RUN cargo build --release

# Etapa 2: Criar a imagem final
FROM debian:buster-slim

# Instalar dependências necessárias
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && apt-get clean

WORKDIR /app

# Copiar o binário compilado da etapa de construção
COPY --from=builder /app/target/release/rest_api_rust .

# Expor a porta que a aplicação usará
EXPOSE 8080

# Definir a variável de ambiente para produção
ENV RUST_LOG=info

# Executar a aplicação
CMD ["./rest_api_rust"]
 