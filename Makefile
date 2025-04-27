.PHONY: test unit integration e2e run-tests

.DEFAULT_GOAL := all

unit:
	@echo "Running unit tests..."
	cargo test --features unit-tests

integration:
	@echo "Running integration tests..."
	cargo test --features integration-tests

e2e:
	@echo "Running e2e tests..."
	cargo test --features e2e-tests

test:
	@echo "Running all tests..."
	cargo test

run-tests: unit integration e2e
	@echo "All tests completed!"
