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

# いったん空でビルドするための準備
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 依存クレートをビルド
RUN cargo build --release

# ソースをコピーして再度ビルド
COPY ./src ./src

# 初回ビルド時に生成されたファイルを削除
RUN rm -f target/release/deps/chatwork_oauth*

# 再度ビルドする
RUN cargo build --release

RUN cargo install --path .
ENTRYPOINT ["chatwork_oauth"]
