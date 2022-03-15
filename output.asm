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
print:
    mov rbp, rsp
    ;--push num--
    push qword[rbp + 16]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--push num--
    push qword[rbp + 16]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax
    ;--while--
loc1:
    ;--dupe--
    push    qword[rsp]
    ;--push 0--
    mov     rax, 0
    push    rax
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
    ;--dupe--
    push    qword[rsp]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
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
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax
    ;--end while--
jmp loc1
loc2:
    ;--pop--
    add rsp, 8
    ;--while--
loc3:
    ;--dupe--
    push    qword[rsp]
    ;--push 0--
    mov     rax, 0
    push    rax
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
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--putc--
    pop rdi
    call putc
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx
    ;--end while--
jmp loc3
loc4:
    ;--end function--
    mov rsp, rbp
    ret
    ;--function--
println:
    mov rbp, rsp
    ;--push num--
    push qword[rbp + 16]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--push num--
    push qword[rbp + 16]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax
    ;--while--
loc5:
    ;--dupe--
    push    qword[rsp]
    ;--push 0--
    mov     rax, 0
    push    rax
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
    jz loc6
    ;--dupe--
    push    qword[rsp]
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
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
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax
    ;--end while--
jmp loc5
loc6:
    ;--pop--
    add rsp, 8
    ;--while--
loc7:
    ;--dupe--
    push    qword[rsp]
    ;--push 0--
    mov     rax, 0
    push    rax
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
    jz loc8
    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx
    ;--putc--
    pop rdi
    call putc
    ;--push 1--
    mov     rax, 1
    push    rax
    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx
    ;--end while--
jmp loc7
loc8:
    ;--push 10--
    mov     rax, 10
    push    rax
    ;--putc--
    pop rdi
    call putc
    ;--end function--
    mov rsp, rbp
    ret
    ;--function--
char_to_int:
    mov rbp, rsp
    ;--push char--
    push qword[rbp + 16]
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--greater than or equal--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setae   al
    movzx   rax, al
    push    rax
    ;--push char--
    push qword[rbp + 16]
    ;--push 57--
    mov     rax, 57
    push    rax
    ;--less than or equal--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setbe   cl
    push    rcx
    ;--and--
    pop rax
    pop rdx
    test rax, rax
    jz loc9
    test rdx, rdx
    mov rax, 1
    jz loc9
    jmp loc10
    push rax
loc9:
    mov rax, 0
loc10:
    push rax
    ;--if--
    pop     rax
    test    rax, rax
    jz      loc11
    ;--push char--
    push qword[rbp + 16]
    ;--push 48--
    mov     rax, 48
    push    rax
    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx
    ;--else--
    jmp     loc12
loc11:
    ;--push 18446744073709551615--
    mov     rax, 18446744073709551615
    push    rax
    ;--end if--
loc12:
    ;--end function--
    mov rax, qword[rsp + 0 * 8]   
    mov qword[rbp - 1 * 8], rax
    mov rsp, rbp
    ret
    ;--function--
main:
    mov rbp, rsp
    ;--dump--
    pop     rdi
    call    dump
    ;--dump--
    pop     rdi
    call    dump
    ;--dump--
    pop     rdi
    call    dump
    ;--dump--
    pop     rdi
    call    dump
    ;--dump--
    pop     rdi
    call    dump
    ;--end function--
    mov rsp, rbp
    ret
