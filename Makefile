default: build run

build:
	cargo build --release

run:
	STORYBUILDER_CLI_SERVER_URL=${SERVER} ./target/release/storybuilder-cli
