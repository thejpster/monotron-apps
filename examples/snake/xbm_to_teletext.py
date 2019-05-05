#!/usr/bin/env python3
import sys

if len(sys.argv) == 1 or sys.argv[1] == '-':
	in_file = sys.stdin
else:
	in_file = open(sys.argv[1])

data = in_file.read()

# TODO read these from file
width_pixels = 96
height_pixels = 39
width_chars = width_pixels // 2
height_chars = height_pixels // 3

print(f"Read {len(data)} bytes")

def reverse(s):
	result = []
	for c in s:
		result.insert(0, c)
	return "".join(result)

pixels = []
for line in data.split("\n"):
	if "0x" in line:
		binary = []
		for piece in line.split():
			piece = int(piece.replace("0x", "").replace(" ", "").replace(",", ""), 16)
			piece = reverse("{:08b}".format(piece))
			piece = piece.replace("1", "x").replace("0", "1").replace("x", "0")
			binary.extend(list(piece))
		print("Read:", "".join(binary))
		pixels.extend(binary)

print(f"num pixels: {len(pixels)}")

sixels = len(pixels) // 6
assert (sixels * 6) == len(pixels)

print(f"num sixels: {sixels}")

def get_pixel(pixels, col, row):
	index = col + (row * width_pixels)
	return pixels[index]

def print_block(b):
	if b == '0':
		sys.stdout.write(".")
	else:
		sys.stdout.write("#")

data = []

for row in range(0, height_pixels):
	for col in range(0, width_pixels):
		print_block(get_pixel(pixels, col, row))
	print()

for row in range(0, height_chars):
	for col in range(0, width_chars):
		pixel_col = col * 2
		pixel_row = row * 3
		bit0 = get_pixel(pixels, pixel_col, pixel_row)
		bit1 = get_pixel(pixels, pixel_col + 1, pixel_row)
		bit2 = get_pixel(pixels, pixel_col, pixel_row + 1)
		bit3 = get_pixel(pixels, pixel_col + 1, pixel_row + 1)
		bit4 = get_pixel(pixels, pixel_col, pixel_row + 2)
		bit5 = get_pixel(pixels, pixel_col + 1, pixel_row + 2)
		bits = bit0 + bit1 + bit2 + bit3 + bit4 + bit5
		data.append(bits)

for row in range(0, height_chars):
	for col in range(0, width_chars):
		b = data[col + (row * width_chars)]
		print_block(b[0])
		print_block(b[1])
	sys.stdout.write("\n")
	for col in range(0, width_chars):
		b = data[col + (row * width_chars)]
		print_block(b[2])
		print_block(b[3])
	sys.stdout.write("\n")
	for col in range(0, width_chars):
		b = data[col + (row * width_chars)]
		print_block(b[4])
		print_block(b[5])
	sys.stdout.write("\n")

numbers = []
for d in data:
	bits = reverse(d)
	number = int(bits, 2)
	numbers.append(number)

print(", ".join(map(lambda x: str(x), numbers)))
