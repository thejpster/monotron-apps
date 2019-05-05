include ../../Common.mk

# Frig this so app always 'builds' (i.e. we use cargo for deps)

$(OUT_DIR)/app:
	cargo build --release --examples --target=thumbv7em-none-eabihf
	cp ./target/thumbv7em-none-eabihf/release/examples/simple $(OUT_DIR)/app

$(OUT_DIR)/linux:
	cargo build --release --examples
	cp ./target/release/examples/simple $(OUT_DIR)/linux
