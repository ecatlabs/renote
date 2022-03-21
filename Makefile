PROJECT := $(shell basename $(CURDIR))
COMMIT := $(shell git rev-parse --short HEAD 2>/dev/null)-$(shell date "+%Y%m%d%H%M%S")
TAG := $(shell git describe --tags --dirty 2>/dev/null)
BUILD_DIR := $(CURDIR)/.target
ARTIFACT_NAME := $(shell $(CURDIR)/hack/generate-artifact-name.sh)
BUILD_CACHE_DIR := $(CURDIR)/.cache

.PHONY: help
help:
	@grep -E '^[a-zA-Z%_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: env
setup-dev: ## Setup development environment
	$(CURDIR)/hack/setup-dev.sh

.PHONY: test
test: ## Run tests
	cargo test $(CARGO_OPTS)

.PHONY: fmt
fmt: ## Format & Lint codes
	rustup component add rustfmt clippy
	cargo fmt

.PHONY: fix
fix:  ## Fix code
	cargo fix --allow-dirty || cargo fix --allow-staged

.PHONY: udep
udep: ## Check undepedencies
	cargo install cargo-udeps --locked
	cargo +nightly udeps

.PHONY: build
build: fmt ## Build binaries
	cargo build $(CARGO_OPTS)

.PHONY: release
release: ## Release binaries
	CARGO_OPTS="--release" $(MAKE) build
	mkdir -p $(BUILD_DIR) && cp $(CURDIR)/target/release/renote $(BUILD_DIR)/$(ARTIFACT_NAME)
	$(MAKE) checksum

.PHONY: checksum
checksum: ## Generate checksum files for built executables
	$(CURDIR)/hack/generate-checksum.sh $(BUILD_DIR)

.PHONY: clean
clean: ## Clean build caches
	cargo clean
	rm -rf $(BUILD_CACHE_DIR) $(BUILD_DIR)
