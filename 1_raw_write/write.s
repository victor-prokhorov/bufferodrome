.section .rodata

stdout_msg_buf:
	.ascii "stdout msg content"
stdout_msg_buf_end:

stderr_msg_buf:
	.ascii "stderr msg content"
stderr_msg_buf_end:

.section .text

.global _start

_start:
	mov  x8, #64
	mov  x0, #1
	adr  x1, stdout_msg_buf
	mov  x2, #(stdout_msg_buf_end - stdout_msg_buf)
	svc  #0

	mov  x8, #64
	mov  x0, #2
	adr  x1, stderr_msg_buf
	mov  x2, #(stderr_msg_buf_end - stderr_msg_buf)
	svc  #0

	mov  x8, #93
	mov  x0, #0
	svc  #0
