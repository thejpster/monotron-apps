#!/usr/bin/python3
import sys

f_in = open(sys.argv[1], "rb")
f_out = open(sys.argv[2], "wb")
last = None
last_count = 1
for b in f_in.read():
    if b == last:
        last_count += 1
    else:
        if last_count > 4:
            f_out.write(bytes([ord('^'), ord('n'), 32 + last_count, last]))
        elif last is not None:
            f_out.write(bytes([last] * last_count))
        last = b
        last_count = 1

