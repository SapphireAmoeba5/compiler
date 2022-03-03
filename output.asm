section .data
section .text
putc:
    mov byte[rsp - 1], dil
    mov rax, 1
    mov rdi, 1
    lea rsi, [rsp - 1]
    mov rdx, 1
    syscall
    ret
dump:
    mov     r9, -3689348814741910323
    sub     rsp, 40
    mov     BYTE [rsp+31], 10
    lea     rcx, [rsp+30]
.L2:
    mov     rax, rdi
    lea     r8, [rsp+32]
    mul     r9
    mov     rax, rdi
    sub     r8, rcx
    shr     rdx, 3
    lea     rsi, [rdx+rdx*4]
    add     rsi, rsi
    sub     rax, rsi
    add     eax, 48
    mov     BYTE [rcx], al
    mov     rax, rdi
    mov     rdi, rdx
    mov     rdx, rcx
    sub     rcx, 1
    cmp     rax, 9
    ja      .L2
    lea     rax, [rsp+32]
    mov     edi, 1
    sub     rdx, rax
    lea     rsi, [rsp+32+rdx]
    mov     rdx, r8
    mov     rax, 1
    syscall
    add     rsp, 40
    ret
global _start
_start:
    ;--push 72--
    push    72
    ;--putc--
    pop rdi
    call putc
    ;--push 101--
    push    101
    ;--putc--
    pop rdi
    call putc
    ;--push 108--
    push    108
    ;--putc--
    pop rdi
    call putc
    ;--push 108--
    push    108
    ;--putc--
    pop rdi
    call putc
    ;--push 111--
    push    111
    ;--putc--
    pop rdi
    call putc
    ;--push 44--
    push    44
    ;--putc--
    pop rdi
    call putc
    ;--push 32--
    push    32
    ;--putc--
    pop rdi
    call putc
    ;--push 87--
    push    87
    ;--putc--
    pop rdi
    call putc
    ;--push 111--
    push    111
    ;--putc--
    pop rdi
    call putc
    ;--push 114--
    push    114
    ;--putc--
    pop rdi
    call putc
    ;--push 108--
    push    108
    ;--putc--
    pop rdi
    call putc
    ;--push 100--
    push    100
    ;--putc--
    pop rdi
    call putc
    ;--push 10--
    push    10
    ;--putc--
    pop rdi
    call putc
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall