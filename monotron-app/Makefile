include ../Common.mk

clean:
	cargo clean

all: $(OUT_DIR)/app

$(OUT_DIR)/app:
	cargo build --release --examples
	cp ./target/thumbv7em-none-eabihf/release/examples/simple $(OUT_DIR)/app
