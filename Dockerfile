FROM rust AS build

WORKDIR /build

ADD amlich ./amlich
ADD vncalendar ./vncalendar
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
RUN mkdir -p ./src/bin

RUN echo "fn main() {}" > ./src/bin/event_consumer.rs
RUN echo "fn main() {}" > ./src/bin/apiserver.rs

RUN cargo build

COPY . .

RUN cargo build --bin apiserver --release

# not working with other slim/alpine dists
FROM ubuntu
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates

RUN update-ca-certificates

COPY --from=build /build/target/release/apiserver /bin/

ENV RUST_PORT=8181
ENV RUST_HOST=0.0.0.0
EXPOSE 8181
CMD [ "/bin/apiserver" ]