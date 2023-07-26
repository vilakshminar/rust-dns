.PHONY: *

all: clean build doc test

clean:
	@echo "Running clean"
	rm -rf \
	./target \
	Cargo.lock

build:
	@echo "*** Building toy dns server"
	cargo build

doc:
	@echo "*** Building documentation"
	cargo doc --no-deps --open

test:
	@echo "*** Testing toy dns server"
	cargo test -- --nocapture
