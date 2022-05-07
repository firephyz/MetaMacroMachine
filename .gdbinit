python
import re
import socket
import sys
import os

def capture_gdb_cmd(cmd):
    gdb_logfile = '/tmp/gdb.log'
    os.system('rm {}'.format(gdb_logfile))
    
    gdb.execute('set logging file {}'.format(gdb_logfile))
    gdb.execute('set logging on')
    gdb.execute(cmd)
    gdb.execute('set logging off')
    
    output_file = open(gdb_logfile)
    output = output_file.read()
    output_file.close()
    return output

def get_frame_depth():
    gdb_info = capture_gdb_cmd('info stack')
    lines = re.split('\n', gdb_info)[:-1]
    depth = re.findall('#([0-9]+).*', lines[-1])[0]
    return depth

def get_frame_call_name():
    gdb_info = capture_gdb_cmd('info frame')
    rip_line = re.split('\n', gdb_info)[:-1][1]
    return re.findall('.*::([^: ]+) .*', rip_line)

def sc():
    call_depth = get_frame_depth()
    step_info = ''
    while get_frame_depth() == call_depth:
        step_info = capture_gdb_cmd('step')

    stack_info = capture_gdb_cmd('info stack')
    call_name = re.findall('#[^ ]+[ ]+([^ ]+).*', re.split('\n', stack_info)[:-1][0])[0]
    print(call_name)
    # gdb.execute('fin')
end

define redir
python
is_done = False
def done():
    global is_done
    is_done = True

sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect('/tmp/gdb-in.sock')
fout = sock.makefile('w')
# fout = open('/tmp/gdb-out.fifo', 'a')
sys.stdout = fout
while not is_done:
    print(' > ', end='')
    fout.flush()
    s = sock.recv(1024)
    # print('Input\n{}'.format(str(s, encoding='ascii')))
    lines = re.split('\n', str(s, encoding='ascii'))[:-1]
    for l in lines:
        # print('Eval \'{}\''.format(l))
        try:
            exec(l)
        except Exception as e:
            print(e)
end
end
    
define sc
python
sc()
end
end

define scn
python
gdb.execute('fin')
sc()
end
end


set pagination off

set substitute-path "/rustc/de1bc0008be096cf7ed67b93402250d3b3e480d0" "/home/kyle/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust"

skip -gfi /home/kyle/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/*
# skip -gfi /rustc/de1bc0008be096cf7ed67b93402250d3b3e480d0/library/*

break syms::main

