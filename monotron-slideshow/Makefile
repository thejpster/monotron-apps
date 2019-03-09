include ../Common.mk

# Frig this so app always 'builds' (i.e. we use cargo for deps)

$(OUT_DIR)/app: ./target/thumbv7em-none-eabihf/release/monotron-slideshow
	cp ./target/thumbv7em-none-eabihf/release/monotron-slideshow $(OUT_DIR)/app

./target/thumbv7em-none-eabihf/release/monotron-slideshow: FORCE
	cargo build --release --all --target=thumbv7em-none-eabihf

$(OUT_DIR)/linux: ./target/release/monotron-slideshow
	cp ./target/release/monotron-slideshow $(OUT_DIR)/linux

./target/release/monotron-slideshow: FORCE
	cargo build --release --all

FORCE:
