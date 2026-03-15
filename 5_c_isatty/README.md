```bash
$ make strace-tty
strace -e trace=write,ioctl ./c_isatty
ioctl(1, TCGETS2, {c_iflag=ICRNL|IXON|IUTF8, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|B38400<<IBSHIFT|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
write(1, "yesline 0\n", 10yesline 0
)             = 10
write(1, "line 1\n", 7line 1
)                 = 7
write(1, "line 2\n", 7line 2
)                 = 7
write(1, "line 3\n", 7line 3
)                 = 7
write(1, "line 4\n", 7line 4
)                 = 7
+++ exited with 0 +++
$
$ make strace-pipe
strace -f -e trace=write,ioctl bash -c './c_isatty | cat'
ioctl(2, TIOCGPGRP, [41325])            = 0
strace: Process 41329 attached
strace: Process 41330 attached
[pid 41329] ioctl(1, TCGETS2, 0xffffcaf8c768) = -1 ENOTTY (Inappropriate ioctl for device)
[pid 41329] write(1, "nonline 0\nline 1\nline 2\nline 3\nl"..., 38) = 38
[pid 41329] +++ exited with 0 +++
[pid 41330] ioctl(1, TCGETS2, {c_iflag=ICRNL|IXON|IUTF8, c_oflag=NL0|CR0|TAB0|BS0|VT0|FF0|OPOST|ONLCR, c_cflag=B38400|B38400<<IBSHIFT|CS8|CREAD, c_lflag=ISIG|ICANON|ECHO|ECHOE|ECHOK|IEXTEN|ECHOCTL|ECHOKE, ...}) = 0
[pid 41330] ioctl(0, TCGETS2, 0xfffffcd5f108) = -1 ENOTTY (Inappropriate ioctl for device)
nonline 0
line 1
line 2
line 3
line 4
[pid 41330] +++ exited with 0 +++
ioctl(2, TIOCGWINSZ, {ws_row=54, ws_col=119, ws_xpixel=1904, ws_ypixel=1728}) = 0
--- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=41329, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
+++ exited with 0 +++
```
