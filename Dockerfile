FROM docker.io/rust:1.65.0 as build-env
RUN apt-get update
RUN apt-get -y install npm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY . /app
WORKDIR /app
RUN cd /app/implementation && cargo build --release
RUN cd /app/frontend && trunk build
RUN cd /app/backend && cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/backend/target/release/backend /

# have to use exec form as we have no shell to execute to execute our binary
CMD ["/backend"]