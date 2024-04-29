.PHONY: all clean install serve rserve build rbuild test help

all: help

clean:
	@echo "Cleaning up generated files..."
	rm -r dist
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

help:
	@echo "Usage: make [target]"
	@echo ""
	@echo "Available targets:"
	@echo "clean            Clean up generated files"
	@echo "install          Install dependencies"
	@echo "serve            Serve the application"
	@echo "rserve           Serve the application in release mode"
	@echo "build            Build the application"
	@echo "rbuild           Build the application in release mode"
	@echo "test             Test the application"
	@echo "help             Display this help message"
