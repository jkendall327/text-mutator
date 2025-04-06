FROM rust:latest

WORKDIR /usr/src/text-mutator
COPY . .

RUN cargo install --path .

CMD ["text-mutator"]
