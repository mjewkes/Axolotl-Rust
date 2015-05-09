# 1 "curve25519-donna-x86-64.s"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 169 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "curve25519-donna-x86-64.s" 2





################################################################################

# 16 "curve25519-donna-x86-64.s"
################################################################################
.text

.extern fmonty

.globl fmul
.globl fsquare
.globl fexpand
.globl fcontract
.globl freduce_coefficients
.globl fscalar
.globl fdifference_backwards
.globl cmult

################################################################################




################################################################################
fmul:




push %rbx
push %r12
push %r13
push %r14
push %r15
push %rdi



mov %rsi,%rcx
mov (%rcx),%rsi
mov 8(%rcx),%r8
mov 16(%rcx),%r9
mov 24(%rcx),%r10
mov 32(%rcx),%r11



mov (%rdx),%rdi
mov 8(%rdx),%r12
mov 16(%rdx),%r13
mov 24(%rdx),%r14
mov 32(%rdx),%r15




































mov %rsi,%rax ; mul %rdi
movq %rax,%xmm0
movq %rdx,%xmm1



mov %rsi,%rax ; mul %r12 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r8,%rax ; mul  %rdi ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm2
movq %rcx,%xmm3



mov %r8,%rax ; mul %r12 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %rsi,%rax ; mul %r13 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r9,%rax ; mul %rdi ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm4
movq %rcx,%xmm5



mov %rsi,%rax ; mul %r14 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r10,%rax ; mul %rdi ; add %rax,%rbx ; adc %rdx,%rcx
mov %r8,%rax ; mul %r13 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r9,%rax ; mul %r12 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm6
movq %rcx,%xmm7



mov %rsi,%rax ; mul %r15 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r11,%rax ; mul %rdi ; add %rax,%rbx ; adc %rdx,%rcx
mov %r10,%rax ; mul %r12 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r8,%rax ; mul %r14 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r9,%rax ; mul %r13 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm8
movq %rcx,%xmm9



mov %r11,%rax ; mul %r12 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r8,%rax ; mul %r15 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r9,%rax ; mul %r14 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r10,%rax ; mul %r13 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm10
movq %rcx,%xmm11



mov %r11,%rax ; mul %r13 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r9,%rax ; mul %r15 ; add %rax,%rbx ; adc %rdx,%rcx
mov %r10,%rax ; mul %r14 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm12
movq %rcx,%xmm13



mov %r10,%rax ; mul %r15 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %r11,%rax ; mul %r14 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm14
movq %rcx,%xmm15



mov %r11,%rax ; mul %r15

donna_reduce:






mov $19,%r15



mov %rdx,%r13
mul %r15
imul %r15,%r13
add %rdx,%r13
mov %rax,%r12



movq %xmm7,%rcx
movq %xmm6,%rbx
add %rbx,%r12
adc %rcx,%r13


# 208 "curve25519-donna-x86-64.s"



movq %xmm14,%rax ; mul %r15 ; movq %xmm15,%r11 ; imul %r15,%r11 ; add %rdx,%r11 ; mov %rax,%r10 ; movq %xmm5,%rcx ; movq %xmm4,%rbx ; add %rbx,%r10 ; adc %rcx,%r11



movq %xmm12,%rax ; mul %r15 ; movq %xmm13,%r9 ; imul %r15,%r9 ; add %rdx,%r9 ; mov %rax,%r8 ; movq %xmm3,%rcx ; movq %xmm2,%rbx ; add %rbx,%r8 ; adc %rcx,%r9



movq %xmm10,%rax ; mul %r15 ; movq %xmm11,%rdi ; imul %r15,%rdi ; add %rdx,%rdi ; mov %rax,%rsi ; movq %xmm1,%rcx ; movq %xmm0,%rbx ; add %rbx,%rsi ; adc %rcx,%rdi



movq %xmm9,%r15
movq %xmm8,%r14




mov $0x7ffffffffffff,%rbx
mov $19,%rcx

coeffreduction:







# 249 "curve25519-donna-x86-64.s"


# 261 "curve25519-donna-x86-64.s"

mov %rsi,%rax ; shr $51,%rsi ; shl $13,%rdi ; or %rsi,%rdi ; add %rdi,%r8 ; adc $0,%r9 ; xor %rdi,%rdi ; mov %rax,%rsi ; and %rbx,%rsi
mov %r8,%rax ; shr $51,%r8 ; shl $13,%r9 ; or %r8,%r9 ; add %r9,%r10 ; adc $0,%r11 ; xor %r9,%r9 ; mov %rax,%r8 ; and %rbx,%r8
mov %r10,%rax ; shr $51,%r10 ; shl $13,%r11 ; or %r10,%r11 ; add %r11,%r12 ; adc $0,%r13 ; xor %r11,%r11 ; mov %rax,%r10 ; and %rbx,%r10
mov %r12,%rax ; shr $51,%r12 ; shl $13,%r13 ; or %r12,%r13 ; add %r13,%r14 ; adc $0,%r15 ; xor %r13,%r13 ; mov %rax,%r12 ; and %rbx,%r12
mov %r14,%rax ; shr $51,%r14 ; shl $13,%r15 ; or %r14,%r15 ; imul $19,%r15 ; add %r15,%rsi ; adc $0,%rdi ; xor %r15,%r15 ; mov %rax,%r14 ; and %rbx,%r14
mov %rsi,%rax ; shr $51,%rsi ; shl $13,%rdi ; or %rsi,%rdi ; add %rdi,%r8 ; adc $0,%r9 ; xor %rdi,%rdi ; mov %rax,%rsi ; and %rbx,%rsi




pop %rdi

mov %rsi,(%rdi)
mov %r8,8(%rdi)
mov %r10,16(%rdi)
mov %r12,24(%rdi)
mov %r14,32(%rdi)

pop %r15
pop %r14
pop %r13
pop %r12
pop %rbx

ret

################################################################################






################################################################################

fsquare:

push %rbx
push %r12
push %r13
push %r14
push %r15
push %rdi



mov %rsi,%rcx
mov (%rcx),%rsi
mov 8(%rcx),%r8
mov 16(%rcx),%r9
mov 24(%rcx),%r10
mov 32(%rcx),%r11




mov %rsi,%rax ; mul %rsi
movq %rax,%xmm0
movq %rdx,%xmm1



mov %rsi,%rax ; mul %r8
sal $1,%rax
rcl $1,%rdx

movq %rax,%xmm2
movq %rdx,%xmm3















mov %r8,%rax ; mul %r8 ; mov %rax,%rbx ; mov %rdx,%rcx
mov %rsi,%rax ; mul %r9 ; sal $1,%rax ; rcl $1,%rdx ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm4
movq %rcx,%xmm5



mov %rsi,%rax ; mul %r10 ; mov %rax,%rbx ; mov %rdx,%rcx ; sal $1,%rbx ; rcl $1,%rcx
mov %r8,%rax ; mul %r9 ; sal $1,%rax ; rcl $1,%rdx ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm6
movq %rcx,%xmm7



mov %rsi,%rax ; mul %r11 ; mov %rax,%rbx ; mov %rdx,%rcx ; sal $1,%rbx ; rcl $1,%rcx
mov %r10,%rax ; mul %r8 ; sal $1,%rax ; rcl $1,%rdx ; add %rax,%rbx ; adc %rdx,%rcx
mov %r9,%rax ; mul %r9 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm8
movq %rcx,%xmm9



mov %r11,%rax ; mul %r8 ; mov %rax,%rbx ; mov %rdx,%rcx ; sal $1,%rbx ; rcl $1,%rcx
mov %r9,%rax ; mul %r10 ; sal $1,%rax ; rcl $1,%rdx ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm10
movq %rcx,%xmm11



mov %r11,%rax ; mul %r9 ; mov %rax,%rbx ; mov %rdx,%rcx ; sal $1,%rbx ; rcl $1,%rcx
mov %r10,%rax ; mul %r10 ; add %rax,%rbx ; adc %rdx,%rcx

movq %rbx,%xmm12
movq %rcx,%xmm13



mov %r10,%rax ; mul %r11
sal $1,%rax
rcl $1,%rdx

movq %rax,%xmm14
movq %rdx,%xmm15



mov %r11,%rax ; mul %r11

jmp donna_reduce

################################################################################

# 439 "curve25519-donna-x86-64.s"
################################################################################

fdifference_backwards:

mov (%rsi),%rax
mov 8(%rsi),%r8
mov 16(%rsi),%r9
mov 24(%rsi),%r10
mov 32(%rsi),%r11

sub (%rdi),%rax
sub 8(%rdi),%r8
sub 16(%rdi),%r9
sub 24(%rdi),%r10
sub 32(%rdi),%r11


mov $0x8000000000000,%rdx

fdifference_backwards_loop:























mov %rax,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%rax ; shr $51,%rcx ; sub %rcx,%r8
mov %r8,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r8 ; shr $51,%rcx ; sub %rcx,%r9
mov %r9,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r9 ; shr $51,%rcx ; sub %rcx,%r10
mov %r10,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r10 ; shr $51,%rcx ; sub %rcx,%r11
mov %r11,%rcx ; sar $63,%rcx ; mov %rcx,%rsi ; and %rdx,%rcx ; add %rcx,%r11 ; and $19,%rsi ; sub %rsi,%rax
mov %rax,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%rax ; shr $51,%rcx ; sub %rcx,%r8
mov %r8,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r8 ; shr $51,%rcx ; sub %rcx,%r9
mov %r9,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r9 ; shr $51,%rcx ; sub %rcx,%r10
mov %r10,%rcx ; sar $63,%rcx ; and %rdx,%rcx ; add %rcx,%r10 ; shr $51,%rcx ; sub %rcx,%r11

mov %rax,(%rdi)
mov %r8,8(%rdi)
mov %r9,16(%rdi)
mov %r10,24(%rdi)
mov %r11,32(%rdi)

ret


################################################################################







################################################################################
fscalar:

mov $121665,%rcx

mov (%rsi),%rax
mul %rcx
shl $13,%rdx
mov %rdx,%r8
mov %rax,%r9

mov 8(%rsi),%rax
mul %rcx
add %r8,%rax
shl $13,%rdx
mov %rdx,%r8
mov %rax,8(%rdi)

mov 16(%rsi),%rax
mul %rcx
add %r8,%rax
shl $13,%rdx
mov %rdx,%r8
mov %rax,16(%rdi)

mov 24(%rsi),%rax
mul %rcx
add %r8,%rax
shl $13,%rdx
mov %rdx,%r8
mov %rax,24(%rdi)

mov 32(%rsi),%rax
mul %rcx
add %r8,%rax
mov %rax,32(%rdi)
shl $13,%rdx
mov $19,%rcx
mov %rdx,%rax
mul %rcx
add %rax,%r9
mov %r9,0(%rdi)

ret

################################################################################



################################################################################
freduce_coefficients:

push %r12

mov $0x7ffffffffffff,%rcx
mov $19,%rdx

mov (%rdi),%r8
mov 8(%rdi),%r9
mov 16(%rdi),%r10
mov 24(%rdi),%r11
mov 32(%rdi),%r12

carrychain_:

mov %r8,%rax
shr $51,%rax
add %rax,%r9
and %rcx,%r8

mov %r9,%rax
shr $51,%rax
add %rax,%r10
and %rcx,%r9

mov %r10,%rax
shr $51,%rax
add %rax,%r11
and %rcx,%r10

mov %r11,%rax
shr $51,%rax
add %rax,%r12
and %rcx,%r11

mov %r12,%rax
shr $51,%rax
imul $19,%rax
add %rax,%r8
and %rcx,%r12

mov %r8,(%rdi)
mov %r9,8(%rdi)
mov %r10,16(%rdi)
mov %r11,24(%rdi)
mov %r12,32(%rdi)

pop %r12
ret


################################################################################



################################################################################
fexpand:

mov $0x7ffffffffffff,%rdx

mov (%rsi),%rax
and %rdx,%rax
mov %rax,(%rdi)

mov 6(%rsi),%rax
shr $3,%rax
and %rdx,%rax
mov %rax,8(%rdi)

mov 12(%rsi),%rax
shr $6,%rax
and %rdx,%rax
mov %rax,16(%rdi)

mov 19(%rsi),%rax
shr $1,%rax
and %rdx,%rax
mov %rax,24(%rdi)

mov 25(%rsi),%rax
shr $4,%rax
and %rdx,%rax
mov %rax,32(%rdi)

ret

################################################################################




################################################################################
fcontract:

mov (%rsi),%rax
mov 8(%rsi),%rdx
mov 16(%rsi),%r8
mov 24(%rsi),%r9
mov 32(%rsi),%r10

mov %rdx,%rcx
shl $51,%rcx
or %rcx,%rax
mov %rax,(%rdi)

shr $13,%rdx
mov %r8,%rcx
shl $38,%rcx
or %rcx,%rdx
mov %rdx,8(%rdi)

shr $26,%r8
mov %r9,%rcx
shl $25,%rcx
or %rcx,%r8
mov %r8,16(%rdi)

shr $39,%r9
shl $12,%r10
or %r10,%r9
mov %r9,24(%rdi)

ret

################################################################################







 






# 746 "curve25519-donna-x86-64.s"

################################################################################
cmult:

push %rbp
push %r13
push %r14

mov %rsp,%rbp
mov $63,%r8
not %r8
and %r8,%rsp

mov %rdx,%r13
mov %rcx,%r14

sub $512,%rsp


movq (%rcx),%rax
movq %rax,(%rsp)
movq 8(%rcx),%r8
movq %r8,8(%rsp)
movq 16(%rcx),%r9
movq %r9,16(%rsp)
movq 24(%rcx),%r10
movq %r10,24(%rsp)
movq 32(%rcx),%r11
movq %r11,32(%rsp)


movq $1,64(%rsp)
movq $0,72(%rsp)
movq $0,80(%rsp)
movq $0,88(%rsp)
movq $0,96(%rsp)


movq $1,128(%rsp)
movq $0,136(%rsp)
movq $0,144(%rsp)
movq $0,152(%rsp)
movq $0,160(%rsp)


movq $0,192(%rsp)
movq $0,200(%rsp)
movq $0,208(%rsp)
movq $0,216(%rsp)
movq $0,224(%rsp)

push %rbx
push %r12
push %r15
push %rdi
push %rsi


# 813 "curve25519-donna-x86-64.s"

mov $256,%r12
mov $32,%rbx

cmult_loop_outer:





sub $8,%rbx
movq (%r13,%rbx),%r15
shl $32,%rbx

or $64,%rbx

cmult_loop_inner:


# 841 "curve25519-donna-x86-64.s"

mov $128,%r8
xor %r9,%r9
bt $63,%r15
cmovc %r8,%r9
mov %r9,%r8
xor $128,%r8

shl $1,%r15

mov %r12,%r11
xor $256,%r11

lea 40(%rsp,%r12),%rdi
mov %rdi,%rsi
lea 40(%rsp,%r11),%rdx
mov %rdx,%rcx
add %r8,%rdi
add %r9,%rsi
add %r8,%rdx
add %r9,%rcx
mov %r14,%r8
call fmonty

xor $256,%r12

dec %rbx
cmp $0,%ebx
jnz cmult_loop_inner

shr $32,%rbx
cmp $0,%rbx
jnz cmult_loop_outer

pop %rsi
pop %rdi
pop %r15
pop %r12
pop %rbx

lea 128(%rsp),%r8

movq (%r8),%rax
movq %rax,(%rdi)
movq 8(%r8),%rax
movq %rax,8(%rdi)
movq 16(%r8),%rax
movq %rax,16(%rdi)
movq 24(%r8),%rax
movq %rax,24(%rdi)
movq 32(%r8),%rax
movq %rax,32(%rdi)

movq 64(%r8),%rax
movq %rax,(%rsi)
movq 72(%r8),%rax
movq %rax,8(%rsi)
movq 80(%r8),%rax
movq %rax,16(%rsi)
movq 88(%r8),%rax
movq %rax,24(%rsi)
movq 96(%r8),%rax
movq %rax,32(%rsi)

mov %rbp,%rsp
pop %r14
pop %r13
pop %rbp

ret

