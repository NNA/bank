# Reading dotenv
DOTENV ?= .env
ifneq (, $(DOTENV))
ifneq (,$(wildcard ./$(DOTENV)))
	include $(DOTENV)
	export
endif
endif

.PHONY: build
build:
	cargo build

.PHONY: test
test:
	cargo test

.PHONY: run
run:
	cargo run -- ./tests/fixtures/missing_column.csv > /tmp/output.txt

.PHONY: tdd
tdd:
	cargo watch -x test