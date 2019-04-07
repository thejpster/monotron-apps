#!/usr/bin/python3
import sys

f_in = open(sys.argv[1], "rb")
f_out = open(sys.argv[2], "wb")
last = None
last_count = 1
for b in f_in.read():
    print("Got byte 0x{:02x}".format(b))
    if b == last:
        print("Dup!")
        last_count += 1
    else:
        if last:
            print("Diff - dumping {} of 0x{:02x}".format(last_count, last))
        if last_count > 4:
            f_out.write(bytes([ord('^'), ord('n'), 32 + last_count, last]))
            # f_out.write("^n{}{}".format(chr(32 + last_count), chr(last)).encode("cp850"))
        elif last is not None:
            f_out.write(bytes([last] * last_count))
        last = b
        last_count = 1

