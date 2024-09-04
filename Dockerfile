FROM rust AS build

WORKDIR /build

ADD amlich ./amlich
ADD vncalendar ./vncalendar
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml
RUN mkdir ./src
RUN echo "fn main() {}" >> ./src/main.rs
# does this give anything?
RUN cargo build

COPY . .
RUN cargo build --bin apiserver --release

# not working with other slim/alpine dists
FROM ubuntu
COPY --from=build /build/target/release/apiserver /bin/

ENV RUST_PORT=8181
ENV RUST_HOST=0.0.0.0
EXPOSE 8181
CMD [ "/bin/apiserver" ]