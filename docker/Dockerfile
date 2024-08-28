
FROM rust:1.80 as builder
WORKDIR /usr/src
# To cache crates index
RUN cargo search regex
RUN git clone https://github.com/osklunds/ImageMapper.git && \
    (cd /usr/src/ImageMapper; git checkout 29f58f3a2e35540f7de3665b8be5ee272ab051c0)
WORKDIR /usr/src/ImageMapper
RUN cargo install --locked --path .

FROM debian:bookworm-slim
RUN uname -m
RUN apt-get update && apt-get install -y procps
COPY --from=builder /usr/local/cargo/bin/image_mapper /usr/local/bin/image_mapper
COPY run.sh .
COPY common.sh .
COPY healthcheck.sh .
CMD ["./run.sh"]

HEALTHCHECK CMD "./healthcheck.sh"
