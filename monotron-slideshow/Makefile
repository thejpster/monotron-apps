EXAMPLES = intro
OUT_DIR = ./bin

BIN_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).bin)
ARM_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example))
LINUX_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example)-linux)

all: directories $(BIN_FILES) $(ARM_FILES) $(LINUX_FILES)

directories: $(OUT_DIR)

$(OUT_DIR):
	@mkdir -p $@

$(OUT_DIR)/%: ./target/thumbv7em-none-eabihf/release/examples/%
	@cp $^ $@

$(OUT_DIR)/%.bin: $(OUT_DIR)/%
	arm-none-eabi-objcopy -O binary $^ $@

./target/thumbv7em-none-eabihf/release/examples/%: FORCE
	cargo build --release --example $* --target=thumbv7em-none-eabihf

$(OUT_DIR)/%-linux: ./target/release/examples/%
	@cp $^ $@

FORCE:
