FROM rust

COPY . /app
RUN cd /app; cargo build
CMD cd /app; cargo run