# Web: Fastfox
In this challenge, contestants are allowed to submit JavaScript that will be 
run in an "outdated browser". The hint tells you exactly which version of 
Firefox that Bob is running, so you can do some experimentation locally. 
There are many publicly available CVEs for this version of Firefox, so 
contestants get their pick of the litter. However, the quality of publicly 
available exploits isn't so hot, so contestants have to do a bit of legwork, 
especially because they don't have debugger access to the system. However, 
a hint does provide them with Bob's libc version to facilitate ROP chains 
and/or GOT overwrites.

I've provided a sample exploit that prints the flag in this repo, although 
my exploit does rely on access to a debugger on Bob's system. You could get 
around this requirement by using JIT-ROP, or looking for gadgets in the 
version of libc specificed in the hint.

Of course, I think everyone solved this challenge with some variation of 
`os.system('cat /flag.txt')`. Woops!

## Prompt
Help me show Bob how slow his browser is!

`http://utctf.live:8124/`

_by mattyp_

## Hint
https://ftp.mozilla.org/pub/firefox/releases/58.0b15/jsshell/

Bob's libc version is `2.27-3ubuntu1.4`.

The flag is stored at `/flag.txt`.

## Flag
`utflag{d1d_y0u_us3_a_j1t_bug_0r_nah}`
