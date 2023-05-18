all: build
install:
	sudo apt install curl build-essential gcc make -y
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source $$HOME/.cargo/env
build:
	cargo build --release
clean:
	cargo clean
	rm -f cnfs/*.err cnfs/*.log
format:
	rustfmt src/*.rs
test: all
	./cnfs/test.sh
.PHONY: all build clean format test