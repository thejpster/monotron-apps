EXAMPLES = intro accu2019 cch2019
OUT_DIR = ./bin
MARKDOWN_FILES = $(foreach _example, $(EXAMPLES), examples/$(_example).md)
COMPRESSED_MARKDOWN_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).md)
BIN_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).bin)
ARM_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).elf)
LINUX_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example)-linux)

.SECONDARY:

all: directories $(BIN_FILES) $(ARM_FILES) $(LINUX_FILES)

directories: $(OUT_DIR)

$(OUT_DIR)/%.md: examples/%.md
	python3 ./compress.py $^ $@

$(OUT_DIR):
	@mkdir -p $@

$(OUT_DIR)/%.elf: ./target/thumbv7em-none-eabihf/release/examples/%
	@cp $^ $@

$(OUT_DIR)/%.bin: $(OUT_DIR)/%.elf
	arm-none-eabi-objcopy -O binary $^ $@

./target/thumbv7em-none-eabihf/release/examples/%: FORCE $(OUT_DIR)/%.md
	cargo build --release --example $* --target=thumbv7em-none-eabihf

./target/release/examples/%: FORCE $(OUT_DIR)/%.md
	cargo build --release --example $*

$(OUT_DIR)/%-linux: ./target/release/examples/%
	@cp $^ $@

FORCE:
