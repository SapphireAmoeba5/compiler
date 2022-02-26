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
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_1
    ;--push 4201--
    push    4201
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_2
    ;--push 4202--
    push    4202
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     jump_addr_3
jump_addr_2:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_3:
    ;--else--
    jmp     jump_addr_4
jump_addr_1:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_4:
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_5
    ;--push 69--
    push    69
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_6
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     jump_addr_7
jump_addr_6:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_7:
    ;--else--
    jmp     jump_addr_8
jump_addr_5:
    ;--push 4203--
    push    4203
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_8:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_9
    ;--push 4204--
    push    4204
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_10
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     jump_addr_11
jump_addr_10:
    ;--push 4205--
    push    4205
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_11:
    ;--else--
    jmp     jump_addr_12
jump_addr_9:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_12:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_13
    ;--push 4206--
    push    4206
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_14
    ;--push 4207--
    push    4207
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_15
    ;--push 4208--
    push    4208
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     jump_addr_16
jump_addr_15:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_16:
    ;--else--
    jmp     jump_addr_17
jump_addr_14:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_17:
    ;--else--
    jmp     jump_addr_18
jump_addr_13:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_18:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_19
    ;--push 4209--
    push    4209
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_20
    ;--push 42010--
    push    42010
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      jump_addr_21
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     jump_addr_22
jump_addr_21:
    ;--push 42011--
    push    42011
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_22:
    ;--else--
    jmp     jump_addr_23
jump_addr_20:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_23:
    ;--else--
    jmp     jump_addr_24
jump_addr_19:
    ;--push 69--
    push    69
    ;--dump--
    pop     rdi
    call    dump
    ;--end--
jump_addr_24:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall