#!/bin/python3

from pwn import *

# connection details
ADDRESS = 'crypto.utctf.live'
PORT = 4354

# implementation specific constants
ENCODING = 'utf-8'

conn = connect(ADDRESS, PORT)

conn.recvline()
conn.recvline()
g = int(conn.recvline().strip()[3:])
p = int(conn.recvline().strip()[3:])
y = int(conn.recvline().strip()[3:])

for _ in range(128):
	conn.sendline(str(1).encode(ENCODING))
	conn.sendline(str(0).encode(ENCODING))
	conn.sendline(str(pow(y, -1, p)).encode(ENCODING))
	conn.sendline(str(0).encode(ENCODING))

for _ in range(128):
	conn.recvline()
	conn.recvline()
	conn.recvline()
	conn.recvline()

conn.recvline()
print(conn.recvline().strip().decode(ENCODING))

conn.close()
