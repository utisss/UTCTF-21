# Sandwiched
* **Event:** UTCTF 2021
* **Problem Type:** Forensics
* **Point Value / Difficulty:**
* **(Optional) Tools Required / Used:**


## Steps
#### Step 1
When you open up the file, you will see that it's just an (almost) empty PDF file, so that's a dead end. If you start to look at the actual bytes of a file with a tool like [`binwalk`](https://github.com/ReFirmLabs/binwalk) or [`xxd`](https://linux.die.net/man/1/xxd), you will see that there are actually a few files in this thing: A bunch of PDFs, what seems to be a JPEG.

#### Step 2
If you extract the JPEG part of the file and try to open it, you will only get the top part of an image. Luckily, further investigation of the file shows us there's actually more JPEG data further down! You can tell which parts of what file start where using the following hex sequences:

- PDF Header: `25 50 44 46 2D`
- PDF Footer: `25 25 45 4F 46`
- JPEG Header: `FF D8 FF`
- JPEG Footer: `FF D9`

Once you have isolated the chunks of JPEG data, you can combine them with a tool like `cat` into a full JPEG and then open it to get the flag: `utflag{file_sandwich_artist}`. I wrote a script that would handle all of this for me:

```python
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
```
