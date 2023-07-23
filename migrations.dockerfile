ARG RUST_VERSION="1.71.0"
ARG DEBIAN_VERSION="bookworm"
ARG CARGO_INSTALL_ROOT="/dist"

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} as build

ARG CARGO_INSTALL_ROOT
ARG SQLX_CLI_VERSION="0.7.1"
RUN \
  cargo install sqlx-cli \
  --no-default-features \
  --features postgres \
  --version "${SQLX_CLI_VERSION}"

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION}

ARG CARGO_INSTALL_ROOT
ARG USER="user"
ARG HOME="/app"
ARG UID="1000"
ARG GID="1000"

RUN \
  addgroup --gid "${GID}" "${USER}" \
  && adduser --disabled-password --gecos "" --home "${HOME}" --ingroup "${USER}" --uid "${UID}" "${USER}"

COPY --from=build "${CARGO_INSTALL_ROOT}/bin/sqlx" "/bin/sqlx"

WORKDIR "${HOME}"
COPY migrations/ migrations/

USER "${USER}"
ENTRYPOINT [ "sqlx" ]
