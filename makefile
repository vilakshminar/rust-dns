.PHONY: *

all: build

clean:
	@echo "Running clean"
	rm -rf \
	./target \
	Cargo.lock

build:
	@echo "*** Building toy dns server"
	cargo build

test:
	@echo "*** Testing toy dns server"
	cargo test
