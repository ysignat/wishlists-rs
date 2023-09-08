ARG VARIANT="bookworm"

FROM mcr.microsoft.com/vscode/devcontainers/rust:${VARIANT}

# Apt-get
ARG POSTGRES_VERSION="15"
RUN \ 
  echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list \
  && wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add - \
  && apt-get update \
  && apt-get install --yes --no-install-recommends postgresql-client-${POSTGRES_VERSION} \
  && apt-get clean --yes \
  && apt-get autoremove -yqq --purge \
  && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/man /usr/share/doc /usr/share/doc-base

# Rust
ARG RUST_VERSION="1.72.0"
RUN rustup toolchain uninstall $(rustup toolchain list)

USER vscode

ARG PATH="/home/vscode/.cargo/bin:${PATH}"
RUN \
  rustup toolchain install "${RUST_VERSION}" --component clippy rustfmt \
  && rustup toolchain install nightly --allow-downgrade --profile minimal --component rustfmt \
  && rustup default "${RUST_VERSION}"

# Cargo
# Trick to update crates.io index in a separate layer
RUN \
  cargo install lazy_static || true

ARG JUST_VERSION="1.14.0"
ARG SEAORM_CLI_VERSION="0.11.3"
ARG CARGO_INFO_VERSION="0.7.3"
ARG CARGO_EXPAND_VERSION="1.0.67"
RUN \
  cargo install cargo-info --version "${CARGO_INFO_VERSION}" \ 
  && cargo install just --version "${JUST_VERSION}" \
  && cargo install sea-orm-cli --version "${SEAORM_CLI_VERSION}" \
  && cargo install cargo-expand --version "${CARGO_EXPAND_VERSION}" \
  && just --completions bash >> ~/.bash_completions \
  && mkdir -p ~/.vscode-server/extensions \
  && printf '\nalias j=just\n\nif [ -r "${HOME}/.bash_completions" ] ; then\n  . "${HOME}/.bash_completions"\nfi\n' >> ~/.profile \
  && printf '\n. ~/.profile\n' >> ~/.zshrc

ENV PGHOST="postgres"
ENV PGPORT="5432"
ENV PGDATABASE="postgres"
ENV PGUSER="postgres"
ENV PGPASSWORD="postgres"
