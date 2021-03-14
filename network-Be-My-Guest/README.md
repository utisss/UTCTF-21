# Samba
* **Event:** UTCTF
* **Problem Type:** Network
* **Point Value / Difficulty:** Medium
* **(Optional) Tools Required / Used:**

## Steps
#### Step 1
First step is preform an nmap scan on the provided IP. nmap -sC -sV should work fine. Once done nmap should reveal that both ports 8881 and 8882 are open and smb is open to guests and should gives the samba version.

#### Step 2
Now, connect to the smb server. smbclient -p 8882 //ip/guest -m SMB3 specifies the verions and works well with how this smb server is configured.

#### Step 3
Once on the server you can enter help and see all the commands availible to you. Running a quick ls will show us that the flag is in this shared folder. Print seems like the right command to use but you'll get NT_STATUS_ACCESS_DENIED. The best command is actually get, which will let you download the file to your system, but there is another way around this, the more command also seems to work.
