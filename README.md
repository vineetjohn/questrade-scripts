# Questrade Scripts

Personal reporting scripts for Questrade, mainly for calculating capital gains in non-registered accounts.

# Requirements
- Rust: https://www.rust-lang.org/tools/install, for building from source.
- Docker: https://docs.docker.com/get-docker/, for usage with docker container.

# Usage

## Build from source
- Clone this repo.
- Register a questrade application and generate a refresh token.
- Copy and update the contents sample `Questrade.toml` config file such that it contains your account details and refresh token.
- Run the application/
  - `cargo run ${PATH_TO_Questrade.toml}`

## Docker Container
- Build the docker image.
  - `docker build -t questrade-scripts .`
- Run the application.
  - `docker run -v ${LOCAL_DIR_CONTAINING_Questrade.toml}:${DOCKER_MOUNT_PATH} -t questrade-scripts:latest /usr/src/questrade-scripts/target/release/questrade-scripts ${DOCKER_MOUNT_PATH}/Questrade.toml`
