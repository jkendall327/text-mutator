# docker build --tag 'rust' . && docker run -p 8080:8080 'rust'
FROM rust:latest

WORKDIR /usr/src/text-mutator
COPY . .

RUN cargo install --path .

CMD ["text-mutator"]