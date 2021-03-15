#!/usr/bin/env python
# -- coding: utf-8 --
"""pomsky v0.0.3
usage: python pomsky.py [options]
Available options are:
  -h        prints help
  -w<FILE>  sets the workingfile, default: /tmp/pomsky.txt
  -p<PORT>  changes the port, default: 8888
  -d<DEBUG> sets the debug command, default: "du -h *"
  -a#<CMD>  sets a command a0 to a9 are free slots to define commands
            default: "ls > /dev/null"
  -v        verbose mode

Example:
    python pomsky.py -w"top.dat" -a0"ps -ax > top.dat" -a1"ls > top.dat"
"""
import os
import socket
import sys

PY2 = True

try:
    # Python 2.x
    from urllib import unquote_plus
    from commands import getstatusoutput as execute
    from BaseHTTPServer import HTTPServer, BaseHTTPRequestHandler
except Exception:
    # Python 3.x
    from urllib.parse import unquote_plus
    from subprocess import getstatusoutput as execute
    from http.server import HTTPServer, BaseHTTPRequestHandler
    PY2 = False

# default values
port, workingfile, debug_cmd, verbose = 8888, "/tmp/pomsky.txt", "du -h *", False
additional_cmds = {"0": "ls > /dev/null"}

# arg parsing
for cwd in sys.argv:
    if cwd.startswith("-h"):
        print(__doc__)
        exit(0)
    if cwd.startswith("-w"):
        workingfile = cwd[2:]
    if cwd.startswith("-p"):
        port = int(cwd[2:])
    if cwd.startswith("-d"):
        debug_cmd = cwd[2:]
    if cwd.startswith("-a") and len(cwd) > 2:
        if cwd[2].isdigit():
            additional_cmds.update({cwd[2]: cwd[3:]})
    if cwd.startswith("-v"):
        verbose = True

# prepare workingfile
if not os.path.isfile(workingfile):
    os.mknod(workingfile)

# create list of html buttons
cmd_buttons = '\n'.join(map(lambda link:
                            """<a href="/run%s"><button>Run %s</button></a>""" % (link[0], link[1]),
                            additional_cmds.items()))

# open socket
HOST, PORT = '', port

def read_content_file():
    """ Read working file """

    f = open(workingfile, "r")
    content = f.read()
    f.close()

    if hasattr(content, "decode"):
        return str(content.decode("utf-8"))
    else:
        return str(content)

def write_content_file(file_content):
    """ Write new working file """

    f = open(workingfile, "w")
    if hasattr(file_content, "decode"):
        f.write(str(file_content.decode("utf-8")))
    else:
        f.write(str(file_content))
    f.close()

def create_response(content, debug, debug_cmd, cmd_buttons=cmd_buttons):
    """ Creates the http response """
    return """\
<html>
<form action="/" method="post">
<textarea name="input" style="width:100%%;height:25%%;" placeholder="%(workingfile)s">%(content)s</textarea>
<input type="submit" value="Submit">
</form>
<hr />
%(cmd_buttons)s
<hr />
<h3>Debug (%(debug_cmd)s):</h3>
<pre>%(debug)s</pre>
</html>""" % {"content": content,
              "debug": debug,
              "debug_cmd": debug_cmd,
              "cmd_buttons": cmd_buttons,
              "workingfile": workingfile}


class RequestHandler(BaseHTTPRequestHandler):

    def do_GET(self):
        redirect = False
        if self.path.startswith("/run"):
            cmd_number = self.path[4]
            os.system("%s &" % additional_cmds[cmd_number])
            redirect = True

        self.create_response(redirect)



    def do_POST(self):
        body_content = self.read_request_body()
        print(body_content)
        if body_content.startswith("input="):
            write_content_file(unquote_plus(body_content[6:]).encode("utf-8"))
        self.create_response(False)

    def read_request_body(self):
        content = self.rfile.read(self.get_length())
        if hasattr(content, "decode"):
            return str(content.decode("utf-8"))
        else:
            return str(content)

    def create_response(self, redirect):
        if redirect:
            self.send_response(302)
            self.send_header('Location', "/?succes=true")
        else:
            self.send_response(200)
            self.send_header("Content-type", "text/html")
        self.end_headers()
        ret, debug = execute(debug_cmd)
        response_body = create_response(read_content_file(), debug, debug_cmd)
        if PY2:
            self.wfile.write(response_body)
        else:
            self.wfile.write(bytes(response_body, 'utf-8'))

    def finish(self):
        if not self.wfile.closed:
            self.wfile.flush()
        self.wfile.close()
        self.rfile.close()

    def get_length(self):
        if PY2:
            content_length = self.headers.getheaders('content-length')
            if content_length:
                length = int(content_length[0])
            else:
                length = 0
        else:
            content_length = self.headers.get('content-length')
            if content_length:
                length = int(content_length)
            else:
                length = 0
        return length


def main():
    print("Serving pomsky on 0.0.0.0 port %s ..." % PORT)
    if verbose:
        print('staring pomsky...\nport:\t\t%s\nworkingfile:\t%s\ncommand:\t%s\ndebug:\t\t%s' % (
    PORT, workingfile, additional_cmds, debug_cmd))
    server = HTTPServer(('', PORT), RequestHandler)
    server.serve_forever()

if __name__ == "__main__":
    main()
