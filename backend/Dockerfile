# docker build --tag 'text-mutator-backend' . && docker run -p 8080:8080 'rust'
FROM rust:latest AS builder

WORKDIR /app
ADD . /app
RUN ls
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/text-mutator /app/
CMD ["/app/text-mutator"]