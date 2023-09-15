.PHONY: build

TEST_KEY := $(shell solana-keygen pubkey ./tests/test-keypairs/test-key.json)

all: build start test stop

build:
	cargo build-bpf
	yarn solita && yarn lint

start:
	solana-test-validator --url https://api.devnet.solana.com \
		--clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s --clone PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT \
		--clone 5qkkEWkKfGEuLZ4iveZzGSHZUM5WqyXTitsVr3TEU1Gp \
		--clone CFDKv9emss3eeYDwGZMPd6CkZXYb9vpYgDGCEwojJp6i \
		--clone cciMwwUJPstviYDc6w5pQkF5x8De12MGkMj54TUB3xS \
		--bpf-program ccsxqYAg64wuLEh45KabyPvkKjrEfDPsDZUQrGn7mf3 ./target/deploy/solana_nft_programs_creator_standard.so \
		--reset --quiet & echo $$! > validator.PID
	sleep 8
	solana airdrop 1000 $(TEST_KEY) --url http://localhost:8899

test:
	yarn test

stop:
	pkill solana-test-validator