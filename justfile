default:
    just -l

run-example:
    cargo run --example example

test-ui:
    cargo test --test ui

clean:
    cargo clean

docs:
    cargo doc --no-deps --open

clippy:
    cargo clippy
    cargo clippy --example example

check-lint: clippy
	cargo fmt -- --check

lint: clippy
	cargo fmt

release:
    cargo publish -p casper-event-standard