CFLAGS = -fno-builtin -std=c11 -Wall -Werror -I..
ARM_CFLAGS = $(CFLAGS) -mcpu=cortex-m4 -nostartfiles -mthumb -Os
POSIX_CFLAGS = $(CFLAGS) -g -D_POSIX_C_SOURCE=199309L

ARM_CC = arm-none-eabi-gcc
OUT_DIR = ./bin
MKDIR_P = mkdir -p
ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))


all: directories $(OUT_DIR)/app.bin $(OUT_DIR)/linux

clean:
	-rm -fr $(OUT_DIR)

rebuild: clean all

$(OUT_DIR)/linux $(OUT_DIR)/app: Makefile $(ROOT_DIR)/monotron-app.ld $(ROOT_DIR)/monotron.h

ifdef SOURCES

ARM_SOURCE := $(SOURCES) $(ROOT_DIR)/lib.c
POSIX_SOURCE := $(SOURCES) $(ROOT_DIR)/lib_posix.c
$(OUT_DIR)/app.bin: $(OUT_DIR)/app
	arm-none-eabi-objcopy -O binary $^ $@

$(OUT_DIR)/app: $(ARM_SOURCE)
	$(ARM_CC) $(ARM_CFLAGS) -T $(ROOT_DIR)/monotron-app.ld -o $(OUT_DIR)/app $(ARM_SOURCE)

$(OUT_DIR)/linux: $(POSIX_SOURCE)
	$(CC) $(POSIX_CFLAGS) -o $(OUT_DIR)/linux $(POSIX_SOURCE)

endif

directories: $(OUT_DIR)

$(OUT_DIR):
	$(MKDIR_P) $(OUT_DIR)

.PHONY: directories clean rebuild all
