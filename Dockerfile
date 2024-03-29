FROM docker.io/rust:1.66.0 as build-env
RUN apt-get update
RUN apt-get -y install npm
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
COPY . /app
WORKDIR /app
RUN cd /app/frontend && TERM=ansi trunk build
RUN cd /app && cargo build -p mikrotik-status --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/mikrotik-status /

# have to use exec form as we have no shell to execute to execute our binary
CMD ["/mikrotik-status"]