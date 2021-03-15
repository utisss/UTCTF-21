# Web: Tar Inspector
This problem links a website that allows you to upload tar archives, 
and then the site will decompress them to show you the directory 
hierarchy within the archive. While this site _shouldn't_ be 
vulnerable to any traditional un-zipping vulnerabilities, like 
path traversal or anything, it is vulnerable to command injection 
because the app uses the `tar` command without properly sanitizing 
filenames. Shell metacharacters are stripped from filenames, so you 
can't do something easy, like upload a file with the name 
`test $(cat /flag.txt).tar`. An additional complication is that 
filenames have a random number appended to them, and the `tar` 
command is a stickler for argument validation.

Instead, you have to google around to find the `--to-command` flag 
for the `tar` command. This command line flag pipes the output of 
decompressed files into a command. Thus, you can upload a tar 
archive, say `flag.tar`, that contains a file `commands.txt`, which 
contains the line `cat /flag.txt`. Once you upload this tar archive, 
you can upload an archive with a malicious filename such as 
`flag__4ed112.tar --to-command bash --exclude .tar`. The 
first argument, `flag__4ed112.tar`, ensures that the previously 
uploaded tar is decompressed as used as input for the command. The 
`--to-command bash` argument pipes the input of `commands.txt` into 
the bash interpreter. Finally, the `--exclude .tar` part ensures 
that the command doesn't fail. With this filename, the directory 
hierarchy should now list an extra entry disclosing the flag.

## Prompt
My friend linked me this cool site. He said 
it's super secure so there's no way you could 
blindly break in.

`http://utctf.live:8123/`

_by mattyp_

## Hint
Flag is stored at /flag.txt

## Flag
`utflag{bl1nd_c0mmand_1nj3ct10n?_n1c3_w0rk}`
