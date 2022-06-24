.PHONY: build check run test clean

build:
	cargo build

check:
	cargo fmt --check
	cargo clippy --tests -- -D clippy::all

run:
	cargo run help

file_path  := dice.png
chunk_type := ruSt
message    := "This is a secret message!"

test: $(file_path)
	@ \
	echo cargo run print $(file_path) && \
	cargo run print $(file_path) && \
	echo && \
	echo cargo run encode $(file_path) $(chunk_type) \"$(message)\" && \
	cargo run encode $(file_path) $(chunk_type) $(message) && \
	echo && \
	echo cargo run decode $(file_path) $(chunk_type) && \
	cargo run decode $(file_path) $(chunk_type) && \
	echo && \
	echo cargo run print $(file_path) && \
	cargo run print $(file_path) && \
	echo && \
	echo cargo run remove $(file_path) $(chunk_type) && \
	cargo run remove $(file_path) $(chunk_type) && \
	echo && \
	echo cargo run print $(file_path) && \
	cargo run print $(file_path)

clean:
	cargo clean
