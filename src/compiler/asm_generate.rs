use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};

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
    push rdx\n"
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
    pop rcx
    pop rax
    div rcx
    push rax\n"
    )
}

pub fn asm_mod(instr: &Instruction) -> String {
    format!(
        "    ;--mod--
    pop rcx
    pop rax
    div rcx
    push rdx\n"
    )
}

pub fn asm_eq(instr: &Instruction) -> String {
    format!(
        "    ;--equals--
    pop rax
    pop rdx
    cmp rax, rdx
    mov rax, 0
    setz al
    push rax\n"
    )
}

pub fn asm_greater_than(instr: &Instruction) -> String {
    format!(
        "    ;--greater than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    seta cl
    push rcx\n"
    )
}

pub fn asm_greater_than_eq(instr: &Instruction) -> String {
    format!(
        "    ;--greater than or equal--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setae cl
    push rcx\n"
    )
}

pub fn asm_less_than(instr: &Instruction) -> String {
    format!(
        "    ;--less than--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setb cl
    push rcx\n"
    )
}

pub fn asm_less_than_eq(instr: &Instruction) -> String {
    format!(
        "    ;--less than or equal--
    xor rcx, rcx
    pop rax
    pop rdx
    cmp rdx, rax
    setbe cl
    push rcx\n"
    )
}

pub fn asm_bitwise_not(instr: &Instruction) -> String {
    format!(
        "    ;--bitwise not--
    pop rax
    not rax
    push rax\n"
    )
}

pub fn asm_bitwise_and(instr: &Instruction) -> String {
    format!(
        "    ;--bitwise and--
    pop rax
    pop rdx
    and rax, rdx
    push rax\n"
    )
}

pub fn asm_bitwise_or(instr: &Instruction) -> String {
    format!(
        "    ;--bitwise or--
    pop rax
    pop rdx
    or rax, rdx
    push rax\n"
    )
}

pub fn asm_bitwise_xor(instr: &Instruction) -> String {
    format!(
        "    ;--bitwise xor--
    pop rax
    pop rdx
    xor rax, rdx
    push rax\n"
    )
}

pub fn asm_not(instr: &Instruction) -> String {
    format!(
        "    ;--not--
    xor rcx, rcx
    pop rax
    test rax, rax
    setz cl
    push rcx\n"
    )
}

pub fn asm_if(instr: &Instruction) -> String {
    format!(
        "    ;--if--
    pop rax
    test rax, rax
    jz jump_addr_{}\n",
        instr.value.clone().unwrap()
    )
}

pub fn asm_else(instr: &Instruction) -> String {
    format!(
        "    ;--else--
    jmp jump_addr_{}
jump_addr_{}:",
        instr.value.clone().unwrap().parse::<u64>().unwrap() + 1,
        instr.value.clone().unwrap()
    )
}

pub fn asm_end(instr: &Instruction) -> String {
    format!(
        "    ;--end--
jump_addr_{}:\n",
        instr.value.clone().unwrap()
    )
}
