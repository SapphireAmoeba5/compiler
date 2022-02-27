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
    ;--push 0--
    push    0
    ;--push 1--
    push    1
    ;--push 93--
    push    93
    ;--while--
jump_addr_1:
    ;--dupe--
    push qword[rsp]
    ;--push 0--
    push    0
    ;--greater than--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    cl
    push    rcx
    ;--do--
    pop rax
    test rax, rax
    jz jump_addr_2
    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx
    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx
    ;--dupe--
    push qword[rsp]
    ;--dump--
    pop     rdi
    call    dump
    ;--dupe--
    push qword[rsp]
    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx
    ;--add--
    pop     rax
    pop     rdx
    add     rax, rdx
    push    rax
    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx
    ;--push 1--
    push    1
    ;--sub--
    pop     rax
    pop     rdx
    sub     rdx, rax
    push    rdx
    ;--end--
    jmp jump_addr_1
jump_addr_2:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall