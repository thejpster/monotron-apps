# Build system template for the Monotron API examples
#
# Copyright (c) Jonathan 'theJPster' Pallant 2019
#
# Available under the Blue Oak Council licence
# (https://blueoakcouncil.org/license/1.0.0)

ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
CRATE_DIR = $(ROOT_DIR)/monotron-app
CFLAGS = -fno-builtin -std=c11 -Wall -Werror -pedantic -I$(CRATE_DIR) -Wl,--gc-sections
ARM_CFLAGS = $(CFLAGS) -mcpu=cortex-m4 -nostartfiles -mthumb -Os
POSIX_CFLAGS = $(CFLAGS) -g -D_POSIX_C_SOURCE=199309L -DLINUX_BUILD -lSDL2 -ldl -lpthread

ARM_CC = arm-none-eabi-gcc
OUT_DIR = ./bin
MKDIR_P = mkdir -p
RUST_LIB = $(CRATE_DIR)/Cargo.toml
LINKER_SCRIPT = $(CRATE_DIR)/monotron-app.ld
HEADER_FILE = $(CRATE_DIR)/monotron.h

all: directories $(OUT_DIR)/$(NAME).bin $(OUT_DIR)/$(NAME)-linux

clean:
	-test -f Cargo.toml && cargo clean || true
	-rm -fr $(OUT_DIR)

rebuild: clean all

$(OUT_DIR)/$(NAME)-linux $(OUT_DIR)/$(NAME).elf: Makefile $(LINKER_SCRIPT) $(HEADER_FILE)

ifdef SOURCES

ARM_SOURCE := $(SOURCES) $(CRATE_DIR)/target/thumbv7em-none-eabi/release/libmonotron_app.a
POSIX_SOURCE := $(SOURCES) $(CRATE_DIR)/target/release/libmonotron_app.a

$(CRATE_DIR)/target/thumbv7em-none-eabi/release/libmonotron_app.a:
	cargo build --release --target=thumbv7em-none-eabi --manifest-path=$(RUST_LIB)

$(CRATE_DIR)/target/release/libmonotron_app.a:
	cargo build --release --manifest-path=$(RUST_LIB)

$(OUT_DIR)/$(NAME).elf: $(ARM_SOURCE)
	$(ARM_CC) -T $(LINKER_SCRIPT) -o $@ $(ARM_SOURCE) $(ARM_CFLAGS)

$(OUT_DIR)/$(NAME)-linux: $(POSIX_SOURCE)
	$(CC) -o $@ $(POSIX_SOURCE) $(POSIX_CFLAGS)

endif

$(OUT_DIR)/%.bin: $(OUT_DIR)/%.elf
	arm-none-eabi-objcopy -O binary $^ $@

directories: $(OUT_DIR)

$(OUT_DIR):
	$(MKDIR_P) $(OUT_DIR)

.PHONY: directories clean rebuild all $(CRATE_DIR)/target/thumbv7em-none-eabi/release/libmonotron_app.a $(CRATE_DIR)/target/release/libmonotron_app.a
