FROM rust:1.73.0-bullseye as Builder

WORKDIR /root/app
COPY --chown=root:root . .

RUN cargo build --release --bin vxbot

FROM debian:bullseye-slim as Runner

COPY --from=Builder --chown=root:root /root/app/target/release/vxbot /usr/local/bin/vxbot

RUN apt-get update && apt-get install -y libssl-dev ca-certificates

RUN useradd --create-home --user-group vxbot
USER vxbot
WORKDIR /home/vxbot

LABEL org.opencontainers.image.source=https://github.com/raiga0310/vxbot

ENTRYPOINT [ "vxbot" ]
