run:
    scripts/start-local.sh

one:
    nodemon -e rs -x "cargo run"

build:
    cargo build --release
