FROM rust:1.48.0-alpine

# Chrome
RUN apk add --update \
        udev \
        ttf-freefont \
        chromium \
        pkgconfig \
        libressl-dev \
        alpine-sdk

WORKDIR /chatwork_oauth

COPY Cargo.toml Cargo.toml
COPY ./src ./src

RUN cargo build --release

RUN cargo install --path .

ENTRYPOINT ["chatwork_oauth"]
