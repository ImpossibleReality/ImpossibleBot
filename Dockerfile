FROM rust as builder

ARG APP_NAME="impossiblebot"
ARG TARGET="aarch64-unknown-linux-gnu"
RUN apt-get update
RUN rustup target add $TARGET
RUN mkdir /usr/src/$APP_NAME
WORKDIR /usr/src/$APP_NAME

RUN apt-get install -y g++-aarch64-linux-gnu libc6-dev-arm64-cross
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER="aarch64-linux-gnu-gcc"
RUN rustup target add $TARGET

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN cargo build --release --target=$TARGET
RUN groupadd -g 10001 -r $APP_NAME
RUN useradd -r -g $APP_NAME -u 10001 $APP_NAME

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM debian:buster-slim
ARG APP_NAME="impossiblebot"
ARG TARGET="aarch64-unknown-linux-gnu"
WORKDIR /user/local/bin/
COPY --from=0 /etc/passwd /etc/passwd
COPY --from=builder /usr/src/$APP_NAME/target/$TARGET/release/$APP_NAME ./impossiblebot
USER $APP_NAME

CMD ["./impossiblebot"]
