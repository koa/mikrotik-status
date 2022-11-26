FROM docker.io/rust:1.65.0 as build-env
RUN apt-get update
RUN apt-get -y install npm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /app
COPY backend /app/backend
COPY frontend /app/frontend
COPY implementation /app/implementation
COPY Cargo.* /app
RUN cargo build -p implementation --release
WORKDIR /app/frontend
RUN trunk build
WORKDIR /app
RUN cargo build -p backend --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/backend /

# have to use exec form as we have no shell to execute to execute our binary
CMD ["/backend"]