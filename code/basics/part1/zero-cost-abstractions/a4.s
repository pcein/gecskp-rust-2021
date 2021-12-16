	.text
	.file	"a40-317d481089b8c8fe83113de504472633.rs"
	.section	.text._ZN2a44main17h7e85f1ac7f351aa1E,"ax",@progbits
	.p2align	4, 0x90
	.type	_ZN2a44main17h7e85f1ac7f351aa1E,@function
_ZN2a44main17h7e85f1ac7f351aa1E:
	.cfi_startproc
	subq	$72, %rsp
.Lcfi0:
	.cfi_def_cfa_offset 80
	movabsq	$5000000050000000, %rax
	movq	%rax, (%rsp)
	movq	%rsp, %rax
	movq	%rax, 8(%rsp)
	movq	_ZN4core3fmt3num50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u64$GT$3fmt17hd8e848a5b9e31113E@GOTPCREL(%rip), %rax
	movq	%rax, 16(%rsp)
	leaq	ref.2(%rip), %rax
	movq	%rax, 24(%rsp)
	movq	$2, 32(%rsp)
	leaq	ref.3(%rip), %rax
	movq	%rax, 40(%rsp)
	movq	$1, 48(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 56(%rsp)
	movq	$1, 64(%rsp)
	leaq	24(%rsp), %rdi
	callq	_ZN3std2io5stdio6_print17h0e1f1f38819db7baE@PLT
	addq	$72, %rsp
	retq
.Lfunc_end0:
	.size	_ZN2a44main17h7e85f1ac7f351aa1E, .Lfunc_end0-_ZN2a44main17h7e85f1ac7f351aa1E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4, 0x90
	.type	main,@function
main:
	.cfi_startproc
	movq	%rsi, %rax
	movslq	%edi, %rsi
	leaq	_ZN2a44main17h7e85f1ac7f351aa1E(%rip), %rdi
	movq	%rax, %rdx
	jmp	_ZN3std2rt10lang_start17hee06cb52bfc299e1E@PLT
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
	.cfi_endproc

	.type	str.0,@object
	.section	.rodata.str.0,"a",@progbits
str.0:
	.size	str.0, 0

	.type	str.1,@object
	.section	.rodata.str.1,"a",@progbits
str.1:
	.byte	10
	.size	str.1, 1

	.type	ref.2,@object
	.section	.data.rel.ro.ref.2,"aw",@progbits
	.p2align	3
ref.2:
	.quad	str.0
	.quad	0
	.quad	str.1
	.quad	1
	.size	ref.2, 32

	.type	ref.3,@object
	.section	.rodata.ref.3,"a",@progbits
	.p2align	3
ref.3:
	.quad	1
	.quad	0
	.quad	3
	.zero	8
	.quad	3
	.zero	8
	.long	32
	.long	0
	.byte	3
	.zero	7
	.size	ref.3, 64


	.section	".note.GNU-stack","",@progbits
