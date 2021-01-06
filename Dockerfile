FROM rust as builder
WORKDIR /usr/src/discord-help-bot
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/discord-help-bot /usr/local/bin/discord-help-bot
CMD ["discord-help-bot"]