from pwn import *
from Crypto.Cipher import AES

# connection details
ADDRESS = 'crypto.utctf.live'
PORT = 4356

# implementation specific constants
ENCODING = 'utf-8'
BLOCK_SIZE = 16

# response values
BAD_PAD_STRING = 'Decryption failed.'
GOOD_PAD_STRING = 'Invalid challenge provided.'


# pad a bytestring using PKCS7
# return a bytestring
def pad(plaintext):
    padding = BLOCK_SIZE - len(plaintext) % BLOCK_SIZE
    padded = plaintext + bytes([padding]) * padding
    return padded


# submit guess
def send_guess(g):
    # only submit the first two blocks of the guess
    # padding only ever exists on the last block
    conn.sendline(bytes(g[:BLOCK_SIZE * 2]).hex())


# check the result of the first remaining unchecked guess
def check_guess():
    # useless info for script
    conn.recvline()
    conn.recvline()
    conn.recvline()

    # result of guess
    result = conn.recvline().decode(ENCODING).strip()

    if result == BAD_PAD_STRING:
        return False
    elif result == GOOD_PAD_STRING:
        return True
    return True


# create a new zero block
def new_block():
    return [0] * BLOCK_SIZE


# guess 256 bytes
# this guessing happens fast because all the guesses are submitted at once
# sending a guess does not wait on the result of the previous guess
# assume only one guess will lead to correct padding
# this assumption usually holds
def calculate_byte(data, pos):
    result = False
    for i in range(0, 256):
        data[pos] = i
        send_guess(data)
    for i in range(0, 256):
        if check_guess():
            data[pos] = i
            result = True
    return result


# figure out the value of the second block after decryption
# store that value in the first block
# start by inserting an empty block
def calculate_block(current):
    # insert block
    current = new_block() + current
    pos = BLOCK_SIZE - 1

    # guess until the first block is complete
    while pos >= 0:
        pad_byte = BLOCK_SIZE - pos

        # set the padding byte for the known bytes
        for i in range(pos + 1, BLOCK_SIZE):
            current[i] ^= pad_byte

        # find the value of the unknown byte
        calculate_byte(current, pos)

        # unset the padding byte for all known bytes
        for i in range(pos, BLOCK_SIZE):
            current[i] ^= pad_byte

        pos -= 1
    return current

# establish connection
conn = connect(ADDRESS, PORT)

# useless info for script
conn.recvline()
conn.recvline()
conn.recvline()
conn.recvline()
conn.recvline()
conn.recvline()

# process the challenge
challenge = bytes.fromhex(conn.recvline().decode(ENCODING).strip()[len('Challenge: '):])
plaintext = pad(challenge)
plaintext = list(plaintext)

# useless info for script
conn.recvline()
conn.recvline()
conn.recvline()

# construct the first block of the answer
answer = new_block()

# while there is challenge bytes left to encrypt
while len(plaintext) > 0:
    # find the decrypted value of the second block
    answer = calculate_block(answer)

    # change the value of the first block
    # so that the second block decrypts to the challenge
    for i in reversed(range(0, BLOCK_SIZE)):
        answer[i] ^= plaintext.pop()


# print results
iv = bytes(answer[:BLOCK_SIZE]).hex()
ciphertext = bytes(answer[BLOCK_SIZE:]).hex()
print('IV: ' + iv)
print('Ciphertext: ' + ciphertext)

# submit the encrypted challenge
conn.sendline(iv + ciphertext)

# useless info for script
conn.recvline()
conn.recvline()
conn.recvline()

# print flag
print(conn.recvline().decode(ENCODING).strip())

# end connection
conn.close()
