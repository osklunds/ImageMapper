
# Building
FROM rust:1.80 as builder

WORKDIR /usr/src
COPY . .
RUN cargo install --locked --path .

# The actual image
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y procps
COPY --from=builder /usr/local/cargo/bin/image_mapper /usr/local/bin/image_mapper
COPY docker/run.sh .
COPY docker/common.sh .
COPY docker/healthcheck.sh .
CMD ["./run.sh"]

HEALTHCHECK CMD "./healthcheck.sh"
