ARG RUST_VERSION="1.72.0"
ARG ALPINE_VERSION="3.18"
ARG CARGO_INSTALL_ROOT="/dist"

FROM rust:${RUST_VERSION}-alpine${ALPINE_VERSION} as build

RUN \
  apk update && apk upgrade \
  && apk add musl-dev

WORKDIR /build

ARG CARGO_INSTALL_ROOT
COPY Cargo.* ./
COPY src/ src/
COPY crates/ crates/ 
RUN \
  cargo install \
  --path . \
  --locked \
  --profile release \
  --target x86_64-unknown-linux-musl

FROM scratch

ARG CARGO_INSTALL_ROOT
COPY --from=build "${CARGO_INSTALL_ROOT}/bin/app" "/app"

ENTRYPOINT [ "/app" ]