section .text
dump:
    mov  r9, -3689348814741910323
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
    ;--push 10--
    push 10
    ;--push 10--
    push 10
    ;--equals--
    pop rax
    pop rdx
    cmp rax, rdx
    mov rax, 0
    setz al
    push rax
;--if--
    pop rax
    test rax, rax
    jz jump_addr_0
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--push 100--
    push 100
    ;--push 100--
    push 100
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--dump--
    pop rdi
    call dump
    ;--push 200--
    push 200
    ;--push 200--
    push 200
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--dump--
    pop rdi
    call dump
;--end--
jump_addr_0:
    ;--push 1000--
    push 1000
    ;--dump--
    pop rdi
    call dump
    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall