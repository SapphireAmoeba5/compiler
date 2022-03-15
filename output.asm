section .data
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
    call main
    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall
    ;--function--
main:
    mov rbp, rsp
    ;--push 3--
    push    3
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
    ;--push 3--
    push    3
    ;--while--
    loc3:
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
    jz loc4
    ;--push 100--
    push    100
    ;--dump--
    pop     rdi
    call    dump
    ;--push 1--
    push    1
    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx
    ;--end while--
jmp loc3
loc4:
    ;--pop--
    add rsp, 8
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
    ;--end function--
    mov rsp, rbp
    ret
