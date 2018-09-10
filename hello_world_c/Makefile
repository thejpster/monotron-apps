all: app.bin

clean:
	-rm -f app app.bin

rebuild: clean all

app.bin: app
	arm-none-eabi-objcopy -O binary ./app ./app.bin

app: main.c monotron-app.ld
	arm-none-eabi-gcc -mcpu=cortex-m4 -nostartfiles main.c -nostdlib -pie -T ./monotron-app.ld -mthumb -o app

.PHONY = clean rebuild all
