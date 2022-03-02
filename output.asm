section .text
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
    ;--push 10--
    push    10
    ;--push 10--
    push    10
    ;--equals--
    pop     rax
    pop     rdx
    cmp     rax, rdx
    mov     rax, 0
    setz    al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc1
    ;--push 1--
    push    1
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc2
loc1:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc2:
    ;--push 10--
    push    10
    ;--push 11--
    push    11
    ;--equals--
    pop     rax
    pop     rdx
    cmp     rax, rdx
    mov     rax, 0
    setz    al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc3
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc4
loc3:
    ;--push 2--
    push    2
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc4:
    ;--push 10--
    push    10
    ;--push 10--
    push    10
    ;--greater than--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc5
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc6
loc5:
    ;--push 3--
    push    3
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc6:
    ;--push 11--
    push    11
    ;--push 10--
    push    10
    ;--greater than--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc7
    ;--push 4--
    push    4
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc8
loc7:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc8:
    ;--push 9--
    push    9
    ;--push 10--
    push    10
    ;--greater than--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc9
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc10
loc9:
    ;--push 5--
    push    5
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc10:
    ;--push 10--
    push    10
    ;--push 10--
    push    10
    ;--greater than or equal--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setae   al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc11
    ;--push 6--
    push    6
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc12
loc11:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc12:
    ;--push 11--
    push    11
    ;--push 10--
    push    10
    ;--greater than or equal--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setae   al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc13
    ;--push 7--
    push    7
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc14
loc13:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc14:
    ;--push 9--
    push    9
    ;--push 10--
    push    10
    ;--greater than or equal--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setae   al
    movzx   rax, al
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc15
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc16
loc15:
    ;--push 8--
    push    8
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc16:
    ;--push 10--
    push    10
    ;--push 10--
    push    10
    ;--less than--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setb    cl
    push    rcx
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc17
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc18
loc17:
    ;--push 9--
    push    9
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc18:
    ;--push 9--
    push    9
    ;--push 10--
    push    10
    ;--less than--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setb    cl
    push    rcx
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc19
    ;--push 10--
    push    10
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc20
loc19:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc20:
    ;--push 11--
    push    11
    ;--push 10--
    push    10
    ;--less than--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setb    cl
    push    rcx
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc21
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc22
loc21:
    ;--push 11--
    push    11
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc22:
    ;--push 10--
    push    10
    ;--not--
    pop     rax
    test    rax, rax
    setz    cl
    movzx rax, cl
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc23
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc24
loc23:
    ;--push 12--
    push    12
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc24:
    ;--push 0--
    push    0
    ;--not--
    pop     rax
    test    rax, rax
    setz    cl
    movzx rax, cl
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc25
    ;--push 13--
    push    13
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc26
loc25:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc26:
    ;--push 1--
    push    1
    ;--not--
    pop     rax
    test    rax, rax
    setz    cl
    movzx rax, cl
    push    rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc27
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc28
loc27:
    ;--push 14--
    push    14
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc28:
    ;--push 500--
    push    500
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc29
    ;--push 15--
    push    15
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc30
loc29:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc30:
    ;--push 103959438--
    push    103959438
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc31
    ;--push 16--
    push    16
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc32
loc31:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc32:
    ;--push 5--
    push    5
    ;--push 3--
    push    3
    ;--and--
    pop rax
    test rax, rax
    jz loc33
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc33
    jmp loc34
    push rax
loc33:
    mov rax, 0
loc34:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc35
    ;--push 17--
    push    17
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc36
loc35:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc36:
    ;--push 1--
    push    1
    ;--push 1--
    push    1
    ;--and--
    pop rax
    test rax, rax
    jz loc37
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc37
    jmp loc38
    push rax
loc37:
    mov rax, 0
loc38:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc39
    ;--push 18--
    push    18
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc40
loc39:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc40:
    ;--push 5--
    push    5
    ;--push 0--
    push    0
    ;--and--
    pop rax
    test rax, rax
    jz loc41
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc41
    jmp loc42
    push rax
loc41:
    mov rax, 0
loc42:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc43
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc44
loc43:
    ;--push 19--
    push    19
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc44:
    ;--push 0--
    push    0
    ;--push 0--
    push    0
    ;--and--
    pop rax
    test rax, rax
    jz loc45
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc45
    jmp loc46
    push rax
loc45:
    mov rax, 0
loc46:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc47
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc48
loc47:
    ;--push 20--
    push    20
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc48:
    ;--push 0--
    push    0
    ;--push 10--
    push    10
    ;--and--
    pop rax
    test rax, rax
    jz loc49
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc49
    jmp loc50
    push rax
loc49:
    mov rax, 0
loc50:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc51
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc52
loc51:
    ;--push 21--
    push    21
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc52:
    ;--push 5--
    push    5
    ;--push 3--
    push    3
    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc54
    pop rdx
    test rdx, rdx
    jnz loc54
loc53:
    mov rax, 0
loc54:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc55
    ;--push 22--
    push    22
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc56
loc55:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc56:
    ;--push 1--
    push    1
    ;--push 1--
    push    1
    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc58
    pop rdx
    test rdx, rdx
    jnz loc58
loc57:
    mov rax, 0
loc58:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc59
    ;--push 23--
    push    23
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc60
loc59:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc60:
    ;--push 5--
    push    5
    ;--push 0--
    push    0
    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc62
    pop rdx
    test rdx, rdx
    jnz loc62
loc61:
    mov rax, 0
loc62:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc63
    ;--push 24--
    push    24
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc64
loc63:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc64:
    ;--push 0--
    push    0
    ;--push 0--
    push    0
    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc66
    pop rdx
    test rdx, rdx
    jnz loc66
loc65:
    mov rax, 0
loc66:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc67
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc68
loc67:
    ;--push 25--
    push    25
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc68:
    ;--push 0--
    push    0
    ;--push 10--
    push    10
    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc70
    pop rdx
    test rdx, rdx
    jnz loc70
loc69:
    mov rax, 0
loc70:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc71
    ;--push 26--
    push    26
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc72
loc71:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc72:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall