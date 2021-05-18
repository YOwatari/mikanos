install:
	rustup component add rust-src
	rustup component add llvm-tools-preview

build:
	cargo bootimage

run:
	cargo run
