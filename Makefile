.PHONY: build

TEST_KEY := $(shell solana-keygen pubkey ./tests/test-key.json)

all: build
build:
	cd program && cargo build-bpf
	cd sdk && yarn && yarn solita
