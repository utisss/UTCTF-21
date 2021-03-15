# Networking: Hack Bob's Box
This problem points you to a couple ports and suggests 
that you should try using `nmap`. Using nmap on these two 
ports should discover `ssh` running on port 8122, and an 
FTP server running on port 8121. The FTP server allows 
read-only access to Bob's home directory. There are various 
red herrings in here, but the next step is in the `.mozilla` 
directory. If you copy Bob's Firefox profile from this 
directory (`/home/bob/.mozilla/firefox/yu85tipn.bob`), you 
can load it in your browser and inspect Bob's history. His 
history will reveal 
`bobsite.com/login?user=bob&pass=i-l0v3-d0lph1n5`. This 
password can then be used to login with `ssh` and reveal 
the flag at `/flag.txt`.

## Prompt
Hack Bob's box!

`nmap` is allowed for this problem only. 
*However*, you may only target `utctf.live:8121` and 
`utctf.live:8122` with `nmap`.

_by mattyp_

## Hints
(none)

## Flag
`utflag{red_teams_are_just_glorified_password_managers}`
