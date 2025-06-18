VERSION=1.0
NAME=rust-practice
EXEC=rust-exec

run:
	@echo "Running debug"
	@cargo run

test:
	@echo "Running test"
	@cargo test

clean:
	@echo "Running clean"
	@cargo clean