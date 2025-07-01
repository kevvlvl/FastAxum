# Build Stage
FROM rust:1.88-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Run Stage
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/FastAxum /app/FastAxum

ARG ADDR
ENV ADDR = ${ADDR}
ARG PORT
ENV PORT = ${PORT}

ENTRYPOINT ["/app/FastAxum"]