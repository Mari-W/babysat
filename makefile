all: build
# installs Rust + necessary build dependencies.
# follow the rust installation instructions until the end
install:
	sudo apt install curl build-essential gcc make -y
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# builds an optimized release binary
build:
	cargo build --release
# removes build artifacts and log files
clean:
	cargo clean
	rm -f cnfs/*.err cnfs/*.log
# runs rustfmt which is installed per default when installing Rust
format:
	rustfmt src/*.rs
# runs all tests.
# run from root directory of this repository.
test: all
	./cnfs/test.sh
.PHONY: all build clean format test