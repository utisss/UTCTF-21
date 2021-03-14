f = open('output.pdf', 'rb').read()

header = b'\x25\x50\x44\x46\x2D'
footer = b'\x25\x25\x45\x4F\x46'

while 1:
    start = f.find(header)
    end = f.find(footer)

    if start >= 0 and end >= 0:
        f = f[:start] + f[end+(len(footer) + 1):]
    else:
        break

out = open('test.jpeg', 'wb')
out.write(f)
out.close()
