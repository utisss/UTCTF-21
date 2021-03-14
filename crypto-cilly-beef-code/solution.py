#!/bin/python3
from math import ceil
from pwn import *

# connection details
ADDRESS = 'crypto.utctf.live'
PORT = 4355

# implementation specific constants
ENCODING = 'utf-8'
BLOCK_SIZE = 16

conn = connect(ADDRESS, PORT)

for i in range(BLOCK_SIZE):
    conn.sendline(b'a' * i)

blocks = len(conn.recvline().strip()) // 2 // BLOCK_SIZE
for i in range(1, BLOCK_SIZE):
    if blocks < len(conn.recvline().strip()) // 2 // BLOCK_SIZE:
        TGT_LENGTH = blocks * BLOCK_SIZE - i - BLOCK_SIZE
        break
for i in range(i + 1, BLOCK_SIZE):
    conn.recvline()


CHARS = [chr(c) for c in range(ord(' '), ord('~'))]

ROUND = 1
CHUNK = 'a' * BLOCK_SIZE
for ROUND in range(ceil(TGT_LENGTH/BLOCK_SIZE)):
    ANSWER = ''
    OFFSET = 0

    for OFFSET in range(BLOCK_SIZE):
        i = -(BLOCK_SIZE - 1 - OFFSET)
        if i == 0:
            START = ''
        else:
            START = CHUNK[i:]
        conn.sendline(START.encode(ENCODING).hex())
        OCHECK = conn.recvline().strip().decode(ENCODING)
        iv1 = bytes.fromhex(OCHECK[(BLOCK_SIZE * 2) * ROUND:(BLOCK_SIZE * 2)*(ROUND+1)])
        CHECK = OCHECK[(BLOCK_SIZE * 2) * (ROUND+1):(BLOCK_SIZE * 2)*(ROUND+2)]
        for i, c in enumerate(CHARS):
            iv = int.from_bytes(bytes.fromhex(OCHECK[0:BLOCK_SIZE * 2]), byteorder='little')
            iv = iv + i + 1
            iv2 = iv.to_bytes(length=BLOCK_SIZE, byteorder='little')
            s = bytes(a ^ b ^ c for (a, b), c in zip(zip(iv1, iv2), (START + ANSWER + c).encode(ENCODING)))
            conn.sendline(s.hex())
        for c in CHARS:
            if CHECK == conn.recvline().strip().decode('utf-8')[2 * BLOCK_SIZE:BLOCK_SIZE * 2 * 2]:
                ANSWER += c
                print(c, end='')
    CHUNK = ANSWER

print()
conn.close()
