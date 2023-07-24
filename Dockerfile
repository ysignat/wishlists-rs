ARG RUST_VERSION="1.71.0"
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
COPY .sqlx/ .sqlx/ 
COPY migrations/ migrations/
RUN \
  cargo install \
  --path . \
  --locked \
  --profile release \
  --target x86_64-unknown-linux-musl

FROM alpine:${ALPINE_VERSION} as release

ARG CARGO_INSTALL_ROOT
ARG USER="user"
ARG HOME="/app"
ARG UID="1000"
ARG GID="1000"

RUN \
  addgroup --gid "${GID}" "${USER}" \
  && adduser --disabled-password --gecos "" --home "${HOME}" --ingroup "${USER}" --uid "${UID}" "${USER}"

COPY --from=build "${CARGO_INSTALL_ROOT}/bin/app" "/bin/app"

USER "${USER}"
WORKDIR "${HOME}"
ENTRYPOINT [ "/bin/app" ]