FROM ubuntu:latest AS prog

RUN apt update && apt install curl build-essential -yq

RUN sh -c "$(curl -sSfL https://release.solana.com/v1.10.31/install)"
RUN sh -c "$(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs)" -- -y
ENV PATH="/root/.local/share/solana/install/active_release/bin:/root/.cargo/bin:$PATH"

# Solana does a questionable download at the beginning of a *first* build-bpf call. Trigger and layer-cache it explicitly.
RUN cargo init --lib /tmp/decoy-crate
WORKDIR /tmp/decoy-crate
RUN echo '[lib]\nname="decoy_crate"\ncrate-type=["cdylib"]' >> /tmp/decoy-crate/Cargo.toml
RUN cargo build-bpf
RUN rm -rf /tmp/decoy-crate

RUN rustup default nightly-2022-12-24

RUN solana config set -u http://localhost:8899
RUN solana-keygen new --no-bip39-passphrase

ADD . /usr/src/app

WORKDIR /usr/src/app

RUN --mount=type=cache,target=/usr/src/app/target \
    --mount=type=cache,target=/usr/lib/cargo/registry \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build-bpf --manifest-path /usr/src/app/program/Cargo.toml \
    && cargo build -p client \
    && cp -r target/debug/client /client \ 
    && cp -r target/deploy/att_state_pda_design.so /program.so

EXPOSE 8899
EXPOSE 9900

ENTRYPOINT ["solana-test-validator", "--bpf-program", "Attest1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o", "/program.so"] #, "--faucet-port", "9900" ]
