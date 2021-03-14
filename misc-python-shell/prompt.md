Upload a Python file that can be run by invoking the python command, and we will run it for you. To do so, send the bytes over the stream, and close the write half. `stdout` will be sent back. For security reasons, we may refuse to run your file if we detect dangerous strings. There is a byte limit and a timeout.

`nc misc.utctf.live 4353`

_by Sohamster_
