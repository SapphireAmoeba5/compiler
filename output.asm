section .data
str1 db "Condition ", 0
str2 db "Not condition ", 0
section .text
; !! COMPILER INTRINSIC !!
strlen:
    xor rax, rax
_strlen_loop:
    cmp byte[rdi + rax], 0
    jz _strlen_end
    add rax, 1
    jmp _strlen_loop
_strlen_end:
    ret
; !! COMPILER INTRINSIC !!
puts:
    call strlen
    mov rdx, rax
    mov rsi, rdi
    mov rax, 1
    mov rdi, 1
    syscall
; !! COMPILER INTRINSIC !!
putc:
    mov byte[rsp - 1], dil
    mov rax, 1
    mov rdi, 1
    lea rsi, [rsp - 1]
    mov rdx, 1
    syscall
    ret
; !! COMPILER INTRINSIC !!
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
    ;--push 6--
    push    6
    ;--dupe--
    push    qword[rsp]
    ;--while--
    loc1:
    ;--dupe--
    push    qword[rsp]
    ;--push 0--
    push    0
    ;--greater than--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    al
    movzx   rax, al
    push    rax
    ;--do--
    pop rax
    test rax, rax
    jz loc2
    ;--over--
    push qword[rsp + 8]
    ;--push 2--
    push    2
    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax
    ;--over--
    push qword[rsp + 8]
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
    jz      loc3
    ;--push string--
    push str1
    ;--puts--
    pop rdi
    call puts
    ;--else--
    jmp     loc4
loc3:
    ;--push string--
    push str2
    ;--puts--
    pop rdi
    call puts
    ;--end if--
loc4:
    ;--push 1--
    push    1
    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx
    ;--end while--
    jmp loc1
loc2:
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall