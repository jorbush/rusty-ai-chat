.PHONY: run, install

run:
	npx tailwindcss -i ./input.css -o ./style/output.css --watch & cargo leptos watch

install:
	rustup toolchain install nightly
	rustup target add wasm32-unknown-unknown
	cargo install trunk cargo-leptos
	npm i
