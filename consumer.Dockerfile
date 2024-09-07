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

RUN cargo build --bin event_consumer --release

# not working with other slim/alpine dists
FROM ubuntu
COPY --from=build /build/target/release/event_consumer /bin/

CMD [ "/bin/event_consumer" ]