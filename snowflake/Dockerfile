FROM rust:latest AS BUILD
USER root
WORKDIR /work/snowflake/
RUN apt-get update -y \
 && DEBIAN_FRONTEND=noninteractive \
    apt-get install --no-install-recommends --assume-yes \
      protobuf-compiler -y
COPY . /work/
RUN cd /work/snowflake/ && cargo build --release

FROM alpine:latest
RUN apk add --no-cache ca-certificates
USER 1000:1000
ENV NODE_ID=0
#RUN mkdir -p /usr/bin/
COPY --link --from=BUILD --chmod=555 --chown=1000:1000 /work/target/release/snowflake /usr/bin/snowflake

CMD ["snowflake"]