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
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc1
    ;--push 1001--
    push    1001
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc2
    ;--push 1002--
    push    1002
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc3
loc2:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc3:
    ;--else--
    jmp     loc4
loc1:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc4:
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc5
    ;--push 0--
    push    0
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc6
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc7
loc6:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc7:
    ;--else--
    jmp     loc8
loc5:
    ;--push 1003--
    push    1003
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc8:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc9
    ;--push 1004--
    push    1004
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc10
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc11
loc10:
    ;--push 1005--
    push    1005
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc11:
    ;--else--
    jmp     loc12
loc9:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc12:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc13
    ;--push 1006--
    push    1006
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc14
    ;--push 1007--
    push    1007
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc15
    ;--push 1008--
    push    1008
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc16
loc15:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc16:
    ;--else--
    jmp     loc17
loc14:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc17:
    ;--else--
    jmp     loc18
loc13:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc18:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc19
    ;--push 1009--
    push    1009
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc20
    ;--push 10010--
    push    10010
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
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
    ;--push 10011--
    push    10011
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc22:
    ;--else--
    jmp     loc23
loc20:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc23:
    ;--else--
    jmp     loc24
loc19:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc24:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc25
    ;--push 10012--
    push    10012
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc26
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
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
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc28:
    ;--else--
    jmp     loc29
loc26:
    ;--push 10013--
    push    10013
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc29:
    ;--else--
    jmp     loc30
loc25:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc30:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc31
    ;--push 10014--
    push    10014
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc32
    ;--push 10015--
    push    10015
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc33
    ;--push 10016--
    push    10016
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc34
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc35
loc34:
    ;--push 10017--
    push    10017
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc36
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc37
loc36:
    ;--push 10018--
    push    10018
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc38
    ;--push 10019--
    push    10019
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc39
    ;--push 10020--
    push    10020
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc40
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc40:
    ;--end if--
loc39:
    ;--else--
    jmp     loc41
loc38:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc41:
    ;--end if--
loc37:
    ;--end if--
loc35:
    ;--else--
    jmp     loc42
loc33:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc42:
    ;--else--
    jmp     loc43
loc32:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc43:
    ;--else--
    jmp     loc44
loc31:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc44:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc45
    ;--push 10021--
    push    10021
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc46
    ;--push 10022--
    push    10022
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc47
    ;--push 10023--
    push    10023
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc48
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc49
loc48:
    ;--push 10024--
    push    10024
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc50
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc51
loc50:
    ;--push 10025--
    push    10025
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc52
    ;--push 10026--
    push    10026
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc53
    ;--push 10027--
    push    10027
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc54
    ;--push 10028--
    push    10028
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc55
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc56
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc56:
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc57
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc57:
    ;--push 10029--
    push    10029
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc58
    ;--push 10030--
    push    10030
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc59
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc60
loc59:
    ;--push 10031--
    push    10031
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc61
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc62
loc61:
    ;--push 10032--
    push    10032
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc63
    ;--push 10033--
    push    10033
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc64
    ;--push 10034--
    push    10034
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc65
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc65:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc66
    ;--push 10035--
    push    10035
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc66:
    ;--push 1--
    push    1
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc67
    ;--push 10036--
    push    10036
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc68
loc67:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc68:
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc69
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--else--
    jmp     loc70
loc69:
    ;--push 10037--
    push    10037
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc70:
    ;--end if--
loc64:
    ;--else--
    jmp     loc71
loc63:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc71:
    ;--end if--
loc62:
    ;--end if--
loc60:
    ;--else--
    jmp     loc72
loc58:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc72:
    ;--else--
    jmp     loc73
loc55:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc73:
    ;--else--
    jmp     loc74
loc54:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc74:
    ;--push 0--
    push    0
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc75
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc75:
    ;--end if--
loc53:
    ;--else--
    jmp     loc76
loc52:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc76:
    ;--end if--
loc51:
    ;--end if--
loc49:
    ;--else--
    jmp     loc77
loc47:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc77:
    ;--else--
    jmp     loc78
loc46:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc78:
    ;--else--
    jmp     loc79
loc45:
    ;--push 0--
    push    0
    ;--dump--
    pop     rdi
    call    dump
    ;--end if--
loc79:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall