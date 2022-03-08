use crate::operation::*;
use crate::{debug_println, error_println, info_println, info_println_if};
use std::collections::HashMap;
use std::hash::Hash;

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

pub struct AsmGenerator {
    data_section: String,
    text_section: String,

    label_id: usize,
    string_id: usize,

    string_map: HashMap<String, usize>,
    block_stack: Vec<Block>,
}

impl AsmGenerator {
    pub fn new() -> Self {
        Self {
            data_section: String::new(),
            text_section: String::new(),

            label_id: 0,
            string_id: 0,

            string_map: HashMap::new(),
            block_stack: Vec::new(),
        }
    }

    pub fn compile_tokens(&mut self, tokens: &Vec<Instruction>) -> Result<String, ()> {
        self.data_section.push_str("section .data\n");

        self.text_section.push_str(
            "section .text
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
_start:\n",
        );
        let mut token_iter = tokens.iter();
        while let Some(instr) = token_iter.next() {
            match instr.op {
                Operation::Push => {
                    self.asm_push(instr);
                }
                Operation::PushString => {
                    self.asm_push_string(instr)?;
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
                Operation::Putc => {
                    self.asm_putc(instr);
                }
                Operation::Puts => {
                    self.asm_puts(instr);
                }
                Operation::Strlen => {
                    self.asm_strlen(instr);
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
                    self.asm_do(instr)?;
                }
                Operation::Else => {
                    self.asm_else(instr)?;
                }
                Operation::End => {
                    self.asm_end(instr);
                }
                Operation::Func => {
                    debug_println!("Function!");
                }
                Operation::Arrow => {
                    debug_println!("Arrow!");
                }
                Operation::In => {
                    debug_println!("In!");
                }
                Operation::Identifier => {
                    debug_println!("Identifier! {}", instr.value);
                }
                _ => {}
            }
        }

        // Push code to return from program
        self.text_section.push_str(
            "    ;--exit program--    
    mov rax, 60
    mov rdi, 0
    syscall",
        );

        if !self.block_stack.is_empty() {
            error_println!("Missing end statement for block");
            return Err(());
        }

        let mut assembly = String::new();
        assembly.push_str(&self.data_section);
        assembly.push_str(&self.text_section);
        Ok(assembly)
    }
}

// Private utility
impl AsmGenerator {
    /// Generates a unique ID for an assembly label
    fn next_label_id(&mut self) -> usize {
        self.label_id += 1;
        return self.label_id;
    }

    /// Generates a unique ID for a string
    fn next_string_id(&mut self) -> usize {
        self.string_id += 1;
        return self.string_id;
    }

    /// Adds a string to the .data section & convert escape sequences. Returns it's ID
    fn add_string_to_data(&mut self, str: &str) -> Result<usize, ()> {
        let id = match self.string_map.contains_key(str) {
            true => {
                return Ok(self.string_map.get(str).unwrap().clone());
            }
            false => {
                let i = self.next_string_id();
                self.string_map.insert(str.to_string(), i);
                i
            }
        };

        let mut parsed_string = String::new();
        let mut escaped = false;
        for ch in str.chars() {
            if escaped {
                let byte = self.char_to_escape_code(ch)?;
                parsed_string.push_str(format!("\", {}, \"", byte).as_str());
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            }
            // Don't push '\n' characters to allow the programmer to format strings over multiple lines
            else if ch != '\n' && ch != '\\' && !escaped {
                parsed_string.push(ch);
            }
        }

        parsed_string.push_str(", 0\n");

        self.data_section
            .push_str(format!("str{} db {}", id, parsed_string).as_str());

        Ok(id)
    }

    fn char_to_escape_code(&self, ch: char) -> Result<u8, ()> {
        match ch {
            '\'' => Ok(0x27),
            '"' => Ok(0x22),
            '?' => Ok(0x3f),
            '\\' => Ok(0x5c),
            'a' => Ok(0x07),
            'b' => Ok(0x08),
            'f' => Ok(0x0c),
            'n' => Ok(0x0a),
            'r' => Ok(0x0d),
            't' => Ok(0x09),
            'v' => Ok(0x0b),
            '0' => Ok(0x00),
            _ => {
                error_println!("\\{} is not a valid escape character", ch);
                Err(())
            }
        }
    }
}

// Assembly generation functions
impl AsmGenerator {
    fn asm_push(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            format!(
                "    ;--push {0}--
    push    {0}\n",
                instr.value
            )
            .as_str(),
        );
    }

    fn asm_push_string(&mut self, instr: &Instruction) -> Result<(), ()> {
        let str_id = self.add_string_to_data(&instr.value)?;

        self.text_section.push_str(
            format!(
                "    ;--push string--
    push str{}\n",
                str_id
            )
            .as_str(),
        );

        Ok(())
    }

    fn asm_dump(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            format!(
                "    ;--dump--
    pop     rdi
    call    dump\n"
            )
            .as_str(),
        );
    }

    fn asm_dupe(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--dupe--
    push    qword[rsp]\n",
        );
    }

    fn asm_pop(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--pop--
    add rsp, 8\n",
        );
    }

    fn asm_swap(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--swap--
    pop rax
    pop rdx
    push rax
    push rdx\n",
        );
    }

    fn asm_over(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--over--
    push qword[rsp + 8]\n",
        );
    }

    fn asm_rot(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--rot--
    pop rax
    pop rdx
    pop rcx
    push rdx
    push rax
    push rcx\n",
        );
    }

    fn asm_putc(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--putc--
    pop rdi
    call putc\n",
        )
    }

    fn asm_puts(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--puts--
    pop rdi
    call puts\n",
        )
    }

    fn asm_strlen(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--strlen--
        pop rdi
        call strlen
        push rax\n",
        )
    }

    fn asm_add(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--add--
    pop rax
    pop rdx
    add rax, rdx
    push rax\n",
        );
    }

    fn asm_sub(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--sub--
    pop rax
    pop rdx
    sub rdx, rax
    push rdx\n",
        );
    }

    fn asm_mul(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--mul--
    pop     rax
    pop     rdx
    mul     rdx
    push    rax\n",
        );
    }

    fn asm_div(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--div--
    pop     rcx
    pop     rax
    xor rdx, rdx
    div     rcx
    push    rax\n",
        );
    }

    fn asm_mod(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--mod--
    pop     rcx
    pop     rax
    xor     rdx, rdx
    div     rcx
    push    rdx\n",
        );
    }

    fn asm_eq(&mut self, instr: &Instruction) {
        self.text_section.push_str(
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
        self.text_section.push_str(
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
        self.text_section.push_str(
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
        self.text_section.push_str(
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
        self.text_section.push_str(
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
        self.text_section.push_str(
            "    ;--bitwise not--
    pop     rax
    not     rax
    push    rax\n",
        )
    }

    fn asm_bitwise_and(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--bitwise and--
    pop     rax
    pop     rdx
    and     rax, rdx
    push    rax\n",
        )
    }

    fn asm_bitwise_or(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--bitwise or--
    pop     rax
    pop     rdx
    or      rax, rdx
    push    rax\n",
        )
    }

    fn asm_bitwise_xor(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--bitwise xor--
        pop     rax
        pop     rdx
        xor     rax, rdx
        push    rax\n",
        )
    }

    fn asm_not(&mut self, instr: &Instruction) {
        self.text_section.push_str(
            "    ;--not--
    pop     rax
    test    rax, rax
    setz    cl
    movzx rax, cl
    push    rax\n",
        )
    }

    fn asm_and(&mut self, instr: &Instruction) {
        let id0 = self.next_label_id();
        let id1 = self.next_label_id();

        self.text_section.push_str(
            format!(
                "    ;--and--
    pop rax
    pop rdx
    test rax, rax
    jz loc{0}
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
        let (id0, id1) = (self.next_label_id(), self.next_label_id());
        self.text_section.push_str(
            format!(
                "    ;--or--
    pop rax
    pop rdx
    test rax, rax
    mov rax, 1
    jnz loc{1}
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
        let id = self.next_label_id();
        self.block_stack.push(Block::new(Operation::If, id));
        self.text_section.push_str(
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
        let id = self.next_label_id();
        self.block_stack.push(Block::new(Operation::While, id));
        self.text_section.push_str(
            format!(
                "    ;--while--
    loc{}:\n",
                id
            )
            .as_str(),
        )
    }

    fn asm_do(&mut self, instr: &Instruction) -> Result<(), ()> {
        match self.block_stack.last() {
            Some(b) => {
                if b.operation != Operation::While {
                    error_println!("Do statement without matching while statement");
                    return Err(());
                }
            }
            None => {
                error_println!("Do statement without matching while statement");
                return Err(());
            }
        }

        let id = self.next_label_id();
        self.block_stack.push(Block::new(Operation::Do, id));
        self.text_section.push_str(
            format!(
                "    ;--do--
    pop rax
    test rax, rax
    jz loc{}\n",
                id
            )
            .as_str(),
        );

        Ok(())
    }

    fn asm_else(&mut self, instr: &Instruction) -> Result<(), ()> {
        let block = match self.block_stack.pop() {
            Some(b) => {
                if b.operation != Operation::If {
                    error_println!("Else statement without matching if statement");
                    return Err(());
                }
                b
            }
            None => {
                error_println!("Else statement without matching if statement");
                return Err(());
            }
        };

        let id = self.next_label_id();
        self.block_stack.push(Block::new(Operation::If, id));

        self.text_section.push_str(
            format!(
                "    ;--else--
    jmp     loc{}
loc{}:\n",
                id, block.block_id
            )
            .as_str(),
        );

        Ok(())
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
                self.text_section.push_str(
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
                self.text_section.push_str(
                    format!(
                        "    ;--end while--
    jmp loc{}
loc{}:\n",
                        while_id.block_id, block.block_id
                    )
                    .as_str(),
                )
            }
            _ => {
                error_println!("Invalid block beginning for end");
                return;
            }
        }
    }

    // TODO: FINISH
    fn asm_func(instr: &Instruction) -> Result<(), ()> {
        Ok(())
    }
}
