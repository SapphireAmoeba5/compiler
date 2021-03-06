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
    ;--push 0--
    mov     rax, 0
    push    rax
    ;--dump--
    pop     rdi
    call    dump
    ;--push 0--
    mov     rax, 0
    push    rax
    ;--while--
loc1:
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--do--
    pop rax
    test rax, rax
    jz loc2
    ;--dupe--
    push    qword[rsp]
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--dupe--
    push    qword[rsp]
    ;--dump--
    pop     rdi
    call    dump
    ;--end while--
jmp loc1
loc2:
    ;--end function--
    mov rsp, rbp
    ret
