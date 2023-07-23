ARG RUST_VERSION="1.71.0"
ARG DEBIAN_VERSION="bookworm"
ARG CARGO_INSTALL_ROOT="/dist"

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} as build

WORKDIR /build

ARG CARGO_INSTALL_ROOT
COPY Cargo.* ./
COPY src/ src/
COPY .sqlx/ .sqlx/ 
RUN \
  cargo install \
  --path . \
  --locked \
  --profile release \
  --target x86_64-unknown-linux-gnu

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