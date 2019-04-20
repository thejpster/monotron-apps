# Build system template for the Monotron API examples
#
# Copyright (c) Jonathan 'theJPster' Pallant 2019
#
# Available under the MIT or Apache 2.0 licence, at your option.

CFLAGS = -fno-builtin -std=c11 -Wall -Werror -pedantic -I.. -Wl,--gc-sections
ARM_CFLAGS = $(CFLAGS) -mcpu=cortex-m4 -nostartfiles -mthumb -Os
POSIX_CFLAGS = $(CFLAGS) -g -D_POSIX_C_SOURCE=199309L -DLINUX_BUILD -lSDL2 -ldl -lpthread

ARM_CC = arm-none-eabi-gcc
OUT_DIR = ./bin
MKDIR_P = mkdir -p
ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
RUST_LIB = $(ROOT_DIR)/monotron-app/Cargo.toml

all: directories $(OUT_DIR)/app.bin $(OUT_DIR)/linux

clean:
	-test -f Cargo.toml && cargo clean || true
	-rm -fr $(OUT_DIR)

rebuild: clean all

$(OUT_DIR)/linux $(OUT_DIR)/app: Makefile $(ROOT_DIR)/monotron-app.ld $(ROOT_DIR)/monotron.h

ifdef SOURCES

ARM_SOURCE := $(SOURCES) $(ROOT_DIR)/monotron-app/target/thumbv7em-none-eabi/release/libmonotron_app.a
POSIX_SOURCE := $(SOURCES) $(ROOT_DIR)/monotron-app/target/release/libmonotron_app.a

$(ROOT_DIR)/monotron-app/target/thumbv7em-none-eabi/release/libmonotron_app.a:
	cargo build --release --target=thumbv7em-none-eabi --manifest-path=$(RUST_LIB)

$(ROOT_DIR)/monotron-app/target/release/libmonotron_app.a:
	cargo build --release --manifest-path=$(RUST_LIB)

$(OUT_DIR)/app: $(ARM_SOURCE)
	$(ARM_CC) -T $(ROOT_DIR)/monotron-app.ld -o $(OUT_DIR)/app $(ARM_SOURCE) $(ARM_CFLAGS)

$(OUT_DIR)/linux: $(POSIX_SOURCE)
	$(CC) -o $(OUT_DIR)/linux $(POSIX_SOURCE) $(POSIX_CFLAGS)

endif

$(OUT_DIR)/app.bin: $(OUT_DIR)/app
	arm-none-eabi-objcopy -O binary $^ $@

directories: $(OUT_DIR)

$(OUT_DIR):
	$(MKDIR_P) $(OUT_DIR)

.PHONY: directories clean rebuild all $(ROOT_DIR)/monotron-app/target/thumbv7em-none-eabi/release/libmonotron_app.a $(ROOT_DIR)/monotron-app/target/release/libmonotron_app.a
