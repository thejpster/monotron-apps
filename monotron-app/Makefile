
SOURCES =
OBJECTS =

include ../Common.mk

rebuild: clean build

clean:
	cargo clean

$(OUT_DIR)/app:
	cargo build --release --examples
	cp ./target/thumbv7em-none-eabihf/release/examples/simple $(OUT_DIR)/app
