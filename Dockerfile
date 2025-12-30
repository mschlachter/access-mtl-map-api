FROM rust:alpine AS builder

COPY ./ /build
RUN cd /build && cargo build --release --verbose

FROM alpine:latest

RUN mkdir /map-api
COPY --from=builder /build/target/release/access-mtl-map-api /map-api/server

RUN adduser -S api_user
USER api_user

HEALTHCHECK --interval=5s --timeout=10s --retries=3 CMD curl --silent --fail http://localhost:$ROCKET_PORT || exit 1
ENTRYPOINT ["/map-api/server"]
