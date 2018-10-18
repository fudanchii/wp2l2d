from debian:stretch-slim
maintainer nurahmadie@gmail.com

run apt update && apt install -y ca-certificates

run addgroup --gid 7000 wp2l2d && \
    adduser \
    --disabled-password \
    --home /home/wp2l2d \
    --shell /bin/sh \
    --uid 7000 \
    --gid 7000 \
    --gecos "" \
    wp2l2d

add target/release/wp2l2d /bin

user wp2l2d

cmd ["wp2l2d"]
