ARG RUST_VERSION="1.71.0"
ARG DEBIAN_VERSION="bookworm"
ARG CARGO_INSTALL_ROOT="/dist"

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} as build

WORKDIR /build

COPY src/ src/
COPY .cargo/ .cargo/
COPY .sqlx/ .sqlx/
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

ARG CARGO_INSTALL_ROOT
ARG BUILD_PROFILE="release"
ARG CARGO_CHANNEL="x86_64-unknown-linux-gnu"
RUN \
  cargo install \
  --path . \
  --all-features \
  --locked \
  --profile "${BUILD_PROFILE}" \
  && cargo build \
  --all-features \
  --locked \
  --profile "${BUILD_PROFILE}"

FROM debian:${DEBIAN_VERSION}-slim as release

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