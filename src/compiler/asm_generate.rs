use crate::operation::*;

/// Places all the boiler plate code in the beginning of the assembly output
pub fn initialize_asm(buf: &mut String) {
    buf.push_str(
        "section .text
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
_start:\n",
    );
}

// Returns assembly for push instruction
pub fn asm_push(instr: &Instruction) -> String {
    format!(
        "    ;--push {0}--
    push {0}\n",
        instr.value.clone().unwrap()
    )
}

pub fn asm_dump(instr: &Instruction) -> String {
    format!(
        "    ;--dump--
    pop rdi
    call dump\n"
    )
}

pub fn asm_add(instr: &Instruction) -> String {
    format!(
        "    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax\n"
    )
}

pub fn asm_sub(instr: &Instruction) -> String {
    format!(
        "    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rbx\n"
    )
}

pub fn asm_mul(instr: &Instruction) -> String {
    format!(
        "    ;--mul--
    pop rax
    pop rdx
    mul rdx
    push rax\n"
    )
}

pub fn asm_div(instr: &Instruction) -> String {
    format!(
        "    ;--div--
    pop rax
    pop rdx
    div rdx
    push rax\n"
    )
}
