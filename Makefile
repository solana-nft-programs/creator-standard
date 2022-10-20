.PHONY: build

TEST_KEY := $(shell solana-keygen pubkey ./sdk/tests/test-keypairs/test-key.json)

all: build start test stop

build:
	cd program && cargo build-bpf
	cd sdk && yarn && yarn solita

start:
	pkill solana-test-validator || true
	solana-test-validator --url https://api.mainnet-beta.solana.com \
		--clone metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s --clone PwDiXFxQsGra4sFFTT8r1QWRMd4vfumiWC1jfWNfdYT \
		--bpf-program mTok58Lg4YfcmwqyrDHpf7ogp599WRhzb6PxjaBqAxS ./program/target/deploy/cardinal_creator_standard.so \
		--reset --quiet & echo $$! > validator.PID
	sleep 8
	solana-keygen pubkey ./sdk/tests/test-keypairs/test-key.json
	solana airdrop 1000 $(TEST_KEY) --url http://localhost:8899

test:
	cd sdk && yarn test

stop:
	pkill solana-test-validator