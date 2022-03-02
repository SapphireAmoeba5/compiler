use std::borrow::Borrow;
use std::iter::Inspect;

use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};

struct Block {
    operation: Operation,
    block_id: usize,
}

impl Block {
    pub fn new(op: Operation, id: usize) -> Self {
        Self {
            operation: op,
            block_id: id,
        }
    }
}

// TODO: Convert functions into methods of Generator struct
pub struct AsmGenerator {
    assembly: String,

    label_id: usize,
    block_stack: Vec<Block>,
}

impl AsmGenerator {
    pub fn new() -> Self {
        Self {
            assembly: String::new(),
            label_id: 0,
            block_stack: Vec::new(),
        }
    }

    pub fn compile_tokens(&mut self, tokens: &Vec<Instruction>) -> String {
        self.assembly.push_str(
            "section .text
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
_start:\n",
        );

        for instr in tokens.iter() {
            match instr.op {
                Operation::Push => {
                    self.asm_push(instr);
                }
                Operation::Dump => {
                    self.asm_dump(instr);
                }
                Operation::Dupe => {
                    self.asm_dupe(instr);
                }
                Operation::Pop => {
                    self.asm_pop(instr);
                }
                Operation::Swap => {
                    self.asm_swap(instr);
                }
                Operation::Over => {
                    self.asm_over(instr);
                }
                Operation::Rot => {
                    self.asm_rot(instr);
                }
                Operation::Add => {
                    self.asm_add(instr);
                }
                Operation::Sub => {
                    self.asm_sub(instr);
                }
                Operation::Mul => {
                    self.asm_mul(instr);
                }
                Operation::Div => {
                    self.asm_div(instr);
                }
                Operation::Mod => {
                    self.asm_mod(instr);
                }
                Operation::Eq => {
                    self.asm_eq(instr);
                }
                Operation::GreaterThan => {
                    self.asm_greater_than(instr);
                }
                Operation::GreaterThanEqual => {
                    self.asm_greater_than_eq(instr);
                }
                Operation::LessThan => {
                    self.asm_less_than(instr);
                }
                Operation::LessThanEqual => {
                    self.asm_less_than_eq(instr);
                }
                Operation::Not => {
                    self.asm_not(instr);
                }
                Operation::And => {
                    self.asm_and(instr);
                }
                Operation::Or => {
                    self.asm_or(instr);
                }
                Operation::BitwiseNot => {
                    self.asm_bitwise_not(instr);
                }
                Operation::BitwiseAnd => {
                    self.asm_bitwise_and(instr);
                }
                Operation::BitwiseOr => {
                    self.asm_bitwise_or(instr);
                }
                Operation::BitwiseXor => {
                    self.asm_bitwise_xor(instr);
                }
                Operation::If => {
                    self.asm_if(instr);
                }
                Operation::While => {
                    self.asm_while(instr);
                }
                Operation::Do => {
                    self.asm_do(instr);
                }
                Operation::Else => {
                    self.asm_else(instr);
                }
                Operation::End => {
                    self.asm_end(instr);
                }
                _ => {}
            }
        }

        // Push code to return from program
        self.assembly.push_str(
            "    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall",
        );

        self.assembly.clone()
    }
}

// Private utility
impl AsmGenerator {
    /// Generates a unique ID for an assembly label
    fn new_label_id(&mut self) -> usize {
        self.label_id += 1;
        return self.label_id;
    }
}

// Assembly generation functions
impl AsmGenerator {
    fn asm_push(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            format!(
                "    ;--push {0}--
    push    {0}\n",
                instr.value
            )
            .as_str(),
        );
    }

    fn asm_dump(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            format!(
                "    ;--dump--
    pop     rdi
    call    dump\n"
            )
            .as_str(),
        );
    }

    fn asm_dupe(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--dupe--
    push    qword[rsp]\n",
        );
    }

    fn asm_pop(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--pop--
    add rsp, 8\n",
        );
    }

    fn asm_swap(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--swap--
    pop rax
    pop rdx
    push pax
    push rdx\n",
        );
    }

    fn asm_over(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--over--
    push qword[rsp + 8]\n",
        );
    }

    fn asm_rot(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx\n",
        );
    }

    fn asm_add(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax\n",
        );
    }

    fn asm_sub(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx\n",
        );
    }

    fn asm_mul(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--mul--
    pop     rax
    pop     rdx
    mul     rdx
    push    rax\n",
        );
    }

    fn asm_div(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax\n",
        );
    }

    fn asm_mod(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx\n",
        );
    }

    fn asm_eq(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--equals--
    pop     rax
    pop     rdx
    cmp     rax, rdx
    mov     rax, 0
    setz    al
    push    rax\n",
        )
    }

    fn asm_greater_than(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--greater than--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    seta    al
    movzx   rax, al
    push    rax\n",
        )
    }

    fn asm_greater_than_eq(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--greater than or equal--
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setae   al
    movzx   rax, al
    push    rax\n",
        )
    }

    fn asm_less_than(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--less than--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setb    cl
    push    rcx\n",
        )
    }

    fn asm_less_than_eq(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--less than or equal--
    xor     rcx, rcx
    pop     rax
    pop     rdx
    cmp     rdx, rax
    setbe   cl
    push    rcx\n",
        )
    }

    fn asm_bitwise_not(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--bitwise not--
    pop     rax
    not     rax
    push    rax\n",
        )
    }

    fn asm_bitwise_and(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--bitwise and--
    pop     rax
    pop     rdx
    and     rax, rdx
    push    rax\n",
        )
    }

    fn asm_bitwise_or(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--bitwise or--
    pop     rax
    pop     rdx
    or      rax, rdx0
    pop     rdx
    xor     rax, rdx
    push    rax\n",
        )
    }

    fn asm_not(&mut self, instr: &Instruction) {
        self.assembly.push_str(
            "    ;--not--
    pop     rax
    test    rax, rax
    setz    cl
    movzx rax, cl
    push    rax\n",
        )
    }

    fn asm_and(&mut self, instr: &Instruction) {
        let id0 = self.new_label_id();
        let id1 = self.new_label_id();

        self.assembly.push_str(
            format!(
                "    ;--and--
    pop rax
    test rax, rax
    jz loc{0}
    pop rdx
    test rdx, rdx
    mov rax, 1
    jz loc{0}
    jmp loc{1}
    push rax
loc{0}:
    mov rax, 0
loc{1}:
    push rax\n",
                id0, id1
            )
            .as_str(),
        )
    }

    fn asm_or(&mut self, instr: &Instruction) {
        let (id0, id1) = (self.new_label_id(), self.new_label_id());
        self.assembly.push_str(
            format!(
                "    ;--or--
    pop rax
    test rax, rax
    mov rax, 1
    jnz loc{1}
    pop rdx
    test rdx, rdx
    jnz loc{1}
loc{0}:
    mov rax, 0
loc{1}:
    push rax\n",
                id0, id1
            )
            .as_str(),
        )
    }

    fn asm_if(&mut self, instr: &Instruction) {
        let id = self.new_label_id();
        self.block_stack.push(Block::new(Operation::If, id));
        self.assembly.push_str(
            format!(
                "    ;--if--
    pop     rax
    test    rax, rax
    jz      loc{}\n",
                id
            )
            .as_str(),
        )
    }

    fn asm_while(&mut self, instr: &Instruction) {
        let id = self.new_label_id();
        self.block_stack.push(Block::new(Operation::While, id));
        self.assembly.push_str(
            format!(
                "    ;--while--
    loc{}:\n",
                id
            )
            .as_str(),
        )
    }

    fn asm_do(&mut self, instr: &Instruction) {
        // TODO: Have this function return an error if there is an error
        match self.block_stack.last() {
            Some(b) => {
                if b.operation != Operation::While {
                    error_println!("Do statement without matching while statement");
                    return;
                }
            }
            None => {
                error_println!("Do statement without matching while statement");
                return;
            }
        }

        let id = self.new_label_id();
        self.block_stack.push(Block::new(Operation::Do, id));
        self.assembly.push_str(
            format!(
                "    ;--do--
    pop rax
    test rax, rax
    jz loc{}\n",
                id
            )
            .as_str(),
        )
    }

    fn asm_else(&mut self, instr: &Instruction) {
        // TODO: Have this function return an error if there is an error
        let block = match self.block_stack.pop() {
            Some(b) => {
                if b.operation != Operation::If {
                    error_println!("Else statement without matching if statement");
                    return;
                }
                b
            }
            None => {
                error_println!("Else statement without matching if statement");
                return;
            }
        };

        let id = self.new_label_id();
        self.block_stack.push(Block::new(Operation::If, id));

        self.assembly.push_str(
            format!(
                "    ;--else--
    jmp     loc{}
loc{}:\n",
                id, block.block_id
            )
            .as_str(),
        )
    }

    fn asm_end(&mut self, instr: &Instruction) {
        let block = match self.block_stack.pop() {
            Some(b) => b,
            None => {
                error_println!("Homeless end");
                return;
            }
        };

        match block.operation {
            Operation::If => {
                self.assembly.push_str(
                    format!(
                        "    ;--end if--
loc{}:\n",
                        block.block_id
                    )
                    .as_str(),
                );
            }
            Operation::Do => {
                // while_id is garunteed to be valid because asm_do will validate it
                let while_id = self.block_stack.pop().unwrap();
                self.assembly.push_str(
                    format!(
                        "    ;--end while--
    jmp loc{}
loc{}:\n",
                        while_id.block_id, block.block_id
                    )
                    .as_str(),
                )
            }
            _ => {}
        }
    }
}
