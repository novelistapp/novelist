FROM ubuntu:18.04
MAINTAINER Katharina Fey <kookie@spacekookie.de>

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    gnome-devel \
    build-essential \
    libgtk-3-dev \
    libcairo-dev

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    curl "$url" -O; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME;

RUN rustup --version; \
    cargo --version; \
    rustc --version;