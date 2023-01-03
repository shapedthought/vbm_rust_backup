FROM rust:latest as builder
WORKDIR /usr/src/vbm_rust_backup
RUN git clone https://github.com/shapedthought/vbm_rust_backup && cd vmb_rust_backup
RUN cargo install --path .

FROM ubuntu
RUN apt-get update && apt-get install wget -y && wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.16_amd64.deb && dpkg -i libssl1.1_1.1.1l-1ubuntu1.2_amd64.deb
COPY --from=builder /usr/local/cargo/bin/vbm_rust_backup /usr/local/bin/vbm_rust_backup