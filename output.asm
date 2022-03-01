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
        jz      jump_addr_1
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
        jz      jump_addr_2
    ;--push 1002--
            push    1002
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_3
    jump_addr_2:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_3:
    ;--else--
        jmp     jump_addr_4
    jump_addr_1:
    ;--push 0--
            push    0
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
    ;--push 0--
            push    0
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_6
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_7
    jump_addr_6:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_7:
    ;--else--
        jmp     jump_addr_8
    jump_addr_5:
    ;--push 1003--
            push    1003
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
        jz      jump_addr_10
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_11
    jump_addr_10:
    ;--push 1005--
            push    1005
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_11:
    ;--else--
        jmp     jump_addr_12
    jump_addr_9:
    ;--push 0--
            push    0
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
        jz      jump_addr_14
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
        jz      jump_addr_15
    ;--push 1008--
            push    1008
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_16
    jump_addr_15:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_16:
    ;--else--
        jmp     jump_addr_17
    jump_addr_14:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_17:
    ;--else--
        jmp     jump_addr_18
    jump_addr_13:
    ;--push 0--
            push    0
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
        jz      jump_addr_20
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
        jz      jump_addr_21
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_22
    jump_addr_21:
    ;--push 10011--
            push    10011
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_22:
    ;--else--
        jmp     jump_addr_23
    jump_addr_20:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_23:
    ;--else--
        jmp     jump_addr_24
    jump_addr_19:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_24:
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_25
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
        jz      jump_addr_26
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
        jz      jump_addr_27
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_28
    jump_addr_27:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_28:
    ;--else--
        jmp     jump_addr_29
    jump_addr_26:
    ;--push 10013--
            push    10013
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_29:
    ;--else--
        jmp     jump_addr_30
    jump_addr_25:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_30:
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_31
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
        jz      jump_addr_32
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
        jz      jump_addr_33
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
        jz      jump_addr_34
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_35
    jump_addr_34:
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
        jz      jump_addr_36
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_37
    jump_addr_36:
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
        jz      jump_addr_38
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
        jz      jump_addr_39
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
        jz      jump_addr_40
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_40:
    ;--end--
            jump_addr_39:
    ;--else--
        jmp     jump_addr_41
    jump_addr_38:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_41:
    ;--end--
            jump_addr_37:
    ;--end--
            jump_addr_35:
    ;--else--
        jmp     jump_addr_42
    jump_addr_33:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_42:
    ;--else--
        jmp     jump_addr_43
    jump_addr_32:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_43:
    ;--else--
        jmp     jump_addr_44
    jump_addr_31:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_44:
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_45
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
        jz      jump_addr_46
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
        jz      jump_addr_47
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
        jz      jump_addr_48
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_49
    jump_addr_48:
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
        jz      jump_addr_50
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_51
    jump_addr_50:
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
        jz      jump_addr_52
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
        jz      jump_addr_53
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
        jz      jump_addr_54
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
        jz      jump_addr_55
    ;--push 0--
            push    0
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_56
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_56:
    ;--push 0--
            push    0
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_57
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_57:
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
        jz      jump_addr_58
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
        jz      jump_addr_59
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_60
    jump_addr_59:
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
        jz      jump_addr_61
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_62
    jump_addr_61:
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
        jz      jump_addr_63
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
        jz      jump_addr_64
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
        jz      jump_addr_65
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_65:
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_66
    ;--push 10035--
            push    10035
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_66:
    ;--push 1--
            push    1
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_67
    ;--push 10036--
            push    10036
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_68
    jump_addr_67:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_68:
    ;--push 0--
            push    0
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_69
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--else--
        jmp     jump_addr_70
    jump_addr_69:
    ;--push 10037--
            push    10037
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_70:
    ;--end--
            jump_addr_64:
    ;--else--
        jmp     jump_addr_71
    jump_addr_63:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_71:
    ;--end--
            jump_addr_62:
    ;--end--
            jump_addr_60:
    ;--else--
        jmp     jump_addr_72
    jump_addr_58:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_72:
    ;--else--
        jmp     jump_addr_73
    jump_addr_55:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_73:
    ;--else--
        jmp     jump_addr_74
    jump_addr_54:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_74:
    ;--push 0--
            push    0
    ;--if--
        pop     rax
        test    rax, rax
        jz      jump_addr_75
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_75:
    ;--end--
            jump_addr_53:
    ;--else--
        jmp     jump_addr_76
    jump_addr_52:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_76:
    ;--end--
            jump_addr_51:
    ;--end--
            jump_addr_49:
    ;--else--
        jmp     jump_addr_77
    jump_addr_47:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_77:
    ;--else--
        jmp     jump_addr_78
    jump_addr_46:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_78:
    ;--else--
        jmp     jump_addr_79
    jump_addr_45:
    ;--push 0--
            push    0
    ;--dump--
        pop     rdi
        call    dump
    ;--end--
            jump_addr_79:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall