FROM rust:latest AS BUILD
WORKDIR /work/snowflake/
RUN apt-get update \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler
COPY . /work/
RUN cargo build --release

FROM alpine:latest
COPY --from=BUILD /work/target/release/snowflake /usr/bin/snowflake
CMD /usr/bin/snowflake