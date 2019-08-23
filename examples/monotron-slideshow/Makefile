EXAMPLES = intro accu2019 cch2019 rustconf2019
OUT_DIR = ./bin
MARKDOWN_FILES = $(foreach _example, $(EXAMPLES), examples/$(_example).md)
COMPRESSED_MARKDOWN_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).md)
BIN_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).bin)
ARM_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example).elf)
LINUX_FILES = $(foreach _example, $(EXAMPLES), $(OUT_DIR)/$(_example)-linux)
V ?= @

all: directories $(BIN_FILES) $(ARM_FILES) $(LINUX_FILES)

.SECONDARY:

.PHONY: all rebuild clean directories $(LINUX_FILES) $(ARM_FILES)

clean:
	rm -rf $(OUT_DIR)
	cargo clean

$(LINUX_FILES): $(COMPRESSED_MARKDOWN_FILES)
$(ARM_FILES): $(COMPRESSED_MARKDOWN_FILES)

rebuild: clean all

directories: $(OUT_DIR)

$(OUT_DIR):
	$(V)mkdir -p $@

$(OUT_DIR)/%.md: examples/%.md
	@echo Compressing $^
	$(V)python3 ./compress.py $^ $@

$(ARM_FILES): $(OUT_DIR)/%.elf: FORCE
	@echo Building $* for Monotron...
	$(V)cargo build --release --example $* --target=thumbv7em-none-eabihf
	$(V)cp ./target/thumbv7em-none-eabihf/release/examples/$* $@

$(OUT_DIR)/%.bin: $(OUT_DIR)/%.elf
	$(V)arm-none-eabi-objcopy -O binary $^ $@

$(LINUX_FILES): $(OUT_DIR)/%-linux: FORCE
	@echo Building $* for Linux...
	$(V)cargo build --release --example $*
	$(V)cp ./target/release/examples/$* $@

FORCE:
