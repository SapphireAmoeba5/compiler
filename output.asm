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
    jz jump_addr_1
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_2
jump_addr_1:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_2:
    ;--push 10--
    push 10
    ;--push 11--
    push 11
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
    jz jump_addr_3
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_4
jump_addr_3:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_4:
    ;--push 10--
    push 10
    ;--push 10--
    push 10
    ;--greater than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    seta cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_5
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_6
jump_addr_5:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_6:
    ;--push 11--
    push 11
    ;--push 10--
    push 10
    ;--greater than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    seta cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_7
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_8
jump_addr_7:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_8:
    ;--push 9--
    push 9
    ;--push 10--
    push 10
    ;--greater than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    seta cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_9
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_10
jump_addr_9:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_10:
    ;--push 10--
    push 10
    ;--push 10--
    push 10
    ;--greater than or equal--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setae cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_11
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_12
jump_addr_11:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_12:
    ;--push 11--
    push 11
    ;--push 10--
    push 10
    ;--greater than or equal--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setae cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_13
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_14
jump_addr_13:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_14:
    ;--push 9--
    push 9
    ;--push 10--
    push 10
    ;--greater than or equal--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setae cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_15
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_16
jump_addr_15:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_16:
    ;--push 10--
    push 10
    ;--push 10--
    push 10
    ;--less than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setb cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_17
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_18
jump_addr_17:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_18:
    ;--push 9--
    push 9
    ;--push 10--
    push 10
    ;--less than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setb cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_19
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_20
jump_addr_19:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_20:
    ;--push 11--
    push 11
    ;--push 10--
    push 10
    ;--less than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setb cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_21
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_22
jump_addr_21:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_22:
    ;--push 10--
    push 10
    ;--not--
    xor rcx, rcx
    pop rax
    test rax, rax
    setz cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_23
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_24
jump_addr_23:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_24:
    ;--push 0--
    push 0
    ;--not--
    xor rcx, rcx
    pop rax
    test rax, rax
    setz cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_25
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_26
jump_addr_25:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_26:
    ;--push 1--
    push 1
    ;--not--
    xor rcx, rcx
    pop rax
    test rax, rax
    setz cl
    push rcx
    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_27
    ;--push 420--
    push 420
    ;--dump--
    pop rdi
    call dump
    ;--else--
    jmp jump_addr_28
jump_addr_27:    ;--push 69--
    push 69
    ;--dump--
    pop rdi
    call dump
    ;--end--
jump_addr_28:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall