CFLAGS = -fno-builtin  -mcpu=cortex-m4 -std=c11 -nostartfiles -mthumb -Wall -Os -Werror -I..
CC = arm-none-eabi-gcc
OUT_DIR = ./bin
MKDIR_P = mkdir -p

all: directories $(OUT_DIR)/app.bin

clean:
	-rm -fr $(OUT_DIR)

rebuild: clean all

$(OUT_DIR)/app.bin: $(OUT_DIR)/app
	arm-none-eabi-objcopy -O binary $^ $@

$(OUT_DIR)/app: $(OUT_DIR)/main.o $(OUT_DIR)/lib.o
	arm-none-eabi-gcc $(CFLAGS) -T ../monotron-app.ld -o $@ $^

$(OUT_DIR)/%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

$(OUT_DIR)/%.o: ../%.c
	$(CC) $(CFLAGS) -c $< -o $@

$(OBJECTS): Makefile ../monotron-app.ld ../monotron.h

directories: $(OUT_DIR)

$(OUT_DIR):
	$(MKDIR_P) $(OUT_DIR)

.PHONY: directories clean rebuild all
