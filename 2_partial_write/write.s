.section .rodata

msg_full:
	.ascii "full"

msg_partial:
	.ascii "partial"

big_buffer: 
	.fill 131072, 1, 'a'

.section .text

.global _start

_start:
	mov x8, #25         
	mov x0, #1          
	mov x1, #3          
	mov x2, #0
	svc #0

	orr x1, x0, #2048   
	mov x2, x1          
	mov x1, #4          
	mov x8, #25         
	svc #0


	mov x8, #64
	mov x0, #1
	ldr x1, =big_buffer
	mov x2, #131072
	svc #0


	cmp x0, #131072
	beq 1f

	mov x8, #64
	mov x0, #2
	adr x1, msg_partial
	mov x2, #7
	svc #0
	b 2f

1: 
	mov x8, #64
	mov x0, #2
	adr x1, msg_full
	mov x2, #4
	svc #0

2: 
	mov x8, #93
	mov x0, #0
	svc #0
