FROM rust AS build

WORKDIR /build

COPY . .
RUN cargo build --bin=apiserver --release
RUN echo $(ls -l /build/target/)
RUN cp ./target/release/apiserver /bin/apiserver

FROM rust AS final
ENV RUST_PORT=8181
ENV RUST_HOST=0.0.0.0
COPY --from=build /bin/apiserver /bin/
EXPOSE 8181
CMD ["/bin/apiserver"]
