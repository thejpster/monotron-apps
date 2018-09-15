CFLAGS = -fno-builtin  -mcpu=cortex-m4 -nostartfiles -nostdlib -mthumb

all: app.bin

clean:
	-rm -f app app.bin

rebuild: clean all

app.bin: app
	arm-none-eabi-objcopy -O binary ./app ./app.bin

app: main.o lib.o ../monotron-app.ld ../monotron.h
	arm-none-eabi-gcc ${CFLAGS} -T ../monotron-app.ld -o app main.o lib.o

main.o: main.c
	arm-none-eabi-gcc ${CFLAGS} -I .. -o main.o -c main.c

lib.o: ../lib.c
	arm-none-eabi-gcc ${CFLAGS} -I .. -o lib.o -c ../lib.c

.PHONY = clean rebuild all
