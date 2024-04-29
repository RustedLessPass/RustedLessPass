.PHONY: all clean install serve

all: install serve

install:
	@echo "Installing dependencies..."
	git submodule update --init --recursive && rustup target add wasm32-unknown-unknown && cargo install --locked trunk

serve:
	clear
	@echo "Serving the application..."
	trunk serve --open

rserve:
	clear
	@echo "release: Serving the application..."
	trunk serve --release --open

build:
	@echo "Building the application..."
	trunk build

rbuild:
	@echo "release: Building the application..."
	trunk build --release

test:
	@echo "Testing the application..."
	cargo test