	.file	"helloworld.c"
	.option nopic
	.attribute arch, "rv64i2p0_m2p0_a2p0_f2p0_d2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-32
	sd	s0,24(sp)
	addi	s0,sp,32
	li	a5,268435456
	sd	a5,-24(s0)
	ld	a5,-24(s0)
	li	a4,72
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,101
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,108
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,108
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,111
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,44
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,32
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,119
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,111
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,114
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,108
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,100
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,33
	sb	a4,0(a5)
	ld	a5,-24(s0)
	li	a4,10
	sb	a4,0(a5)
	li	a5,0
	mv	a0,a5
	ld	s0,24(sp)
	addi	sp,sp,32
	jr	ra
	.size	main, .-main
	.ident	"GCC: (GNU) 9.2.0"
