FROM rust
EXPOSE 8080
COPY . /app
RUN cd /app; cargo build
CMD cd /app; cargo run