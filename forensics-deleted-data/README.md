# Doubly Deleted Data
* **Event:** UTCTF 2021
* **Problem Type:** Forensics
* **Point Value / Difficulty:**
* **(Optional) Tools Required / Used:** testdisk

## Steps
#### Step 1
First of all, you'll want to recover the deleted partition from the "flash drive." Using testdisk, (`testdisk flash_drive.img` and then follow all the default selections), you should be able to recover the parition.

#### Step 2
From there, you can apply the same testdisk technique to analyze all the files in the hacker's home directory. You may not necessarily be able to recover the flag this way since testdisk doesn't work as well for ext4 filesystems, but you may stumble upon some other useful information, like the .bash_log, which contains the command the hacker used to create the flag (not to mention the flag itself)
