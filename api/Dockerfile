# Build
FROM rust:1.46 as build
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /usr/src/truist

# Create pseudo project to use layer cache w/ no dependency changes.
# @hack - Add the missing lib file so pseudo compile passes.
RUN USER=root cargo new --bin api && \
    touch /usr/src/truist/api/src/lib.rs
WORKDIR /usr/src/truist/api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release && \
    rm src/*.rs

# Compile our application.
COPY . .
RUN rm ./target/release/deps/server* ./target/release/deps/api*
RUN cargo build --release

# # # Distribute
# # FROM gcr.io/distroless/cc-debian10
# # COPY --from=build /usr/src/truist/target/release/server /usr/local/bin/server
# # EXPOSE 8000

# # CMD ["server"]