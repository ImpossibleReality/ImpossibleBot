FROM arm64v8/rust:1.54 as builder
WORKDIR /usr/src/impossiblebot
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update # && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/impossiblebot /usr/local/bin/impossiblebot
LABEL com.centurylinklabs.watchtower.enable="true"
CMD ["impossiblebot"]