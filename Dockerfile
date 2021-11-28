FROM rustembedded/cross:aarch64-unknown-linux-gnu-0.2.1
RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libfoo:arm64
    
ARG APP_NAME="impossiblebot"
ARG TARGET="aarch64-unknown-linux-gnu"
ARG GITHUB_SSH_KEY=""
RUN apt-get update
RUN rustup target add $TARGET
RUN mkdir /usr/src/$APP_NAME
WORKDIR /usr/src/$APP_NAME

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

ENV CROSS_DOCKER_IN_DOCKER=true
RUN cargo install cross

RUN cross build --release --target=$TARGET
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
