OUT_DIR = ./bin
BIN_FILES = $(OUT_DIR)/chunky.bin
ARM_FILES = $(OUT_DIR)/chunky.elf
LINUX_FILES = $(OUT_DIR)/chunky-linux
V ?= @

all: directories $(BIN_FILES) $(ARM_FILES) $(LINUX_FILES)

.SECONDARY:

.PHONY: all rebuild clean directories $(LINUX_FILES) $(ARM_FILES)

clean:
	rm -rf $(OUT_DIR)
	cargo clean

rebuild: clean all

directories: $(OUT_DIR)

$(OUT_DIR):
	$(V)mkdir -p $@

$(ARM_FILES): $(OUT_DIR)/%.elf: FORCE
	@echo Building $* for Monotron...
	$(V)cargo build --release --bin $* --target=thumbv7em-none-eabihf
	$(V)cp ./target/thumbv7em-none-eabihf/release/$* $@

$(OUT_DIR)/%.bin: $(OUT_DIR)/%.elf
	$(V)arm-none-eabi-objcopy -O binary $^ $@

$(LINUX_FILES): $(OUT_DIR)/%-linux: FORCE
	@echo Building $* for Linux...
	$(V)cargo build --release --bin $*
	$(V)cp ./target/release/$* $@

FORCE:
