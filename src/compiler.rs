use std::{fs, io::Write};

use super::lexer::{Token, Tokens};
use super::DEFAULT_CELL_COUNT;

#[derive(Debug, Clone)]
pub struct Compiler {
    tokens: Vec<(Token, u8)>,
    cell_count: usize,
    loop_idx: usize, // required to give names to the loop labels
    compiled_source: String,
    compiled: bool
}

impl Compiler {
    pub fn new(tokens: Tokens) -> Self {
        Self {
            tokens: Self::preprocess_tokens(tokens),
            compiled_source: Self::init(DEFAULT_CELL_COUNT),
            cell_count: DEFAULT_CELL_COUNT,
            loop_idx: 0,
            compiled: false
        }
    }

    pub fn with_cells(tokens: Tokens, cell_count: usize) -> Self {
        Self {
            tokens: Self::preprocess_tokens(tokens),
            compiled_source: Self::init(cell_count),
            cell_count,
            loop_idx: 0,
            compiled: false
        }
    }

    pub fn get_assembly(&mut self) -> &String {
        (!self.compiled).then(|| { self.compiled = true; self.compile(); });
        &self.compiled_source
    }

    pub fn save_assembly_output(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut f = fs::File::create(file_path)?;
        f.write_all(self.get_assembly().as_bytes())?;
        f.flush()?;

        Ok(())
    }

    fn compile(&mut self) {
        for (token, occur) in self.tokens.iter() {
            self.compiled_source += match token {
                Token::Increment  => self.increment(*occur),
                Token::Decrement  => self.decrement(*occur),
                Token::Input      => self.input(),
                Token::Output     => self.output(),
                Token::ShiftLeft  => self.shift_left(),
                Token::ShiftRight => self.shift_right(),

                Token::LoopStart  => { self.loop_idx += 1; self.loop_start() },
                Token::LoopEnd    => self.loop_end(),

                _ => unreachable!(),
            }.as_str();
        }

        self.compiled_source += self.error_messages().as_str();
    }

    fn preprocess_tokens(tokens: Tokens) -> Vec<(Token, u8)> {
        let mut v = Vec::new();

        for itm in tokens.0.iter() {
            v.push((*itm, 1));
        }

        v
    }

    fn init(cell_count: usize) -> String {
        format!(
            r#"section .data
    array times {} db 0
    newline db 0xA
    buffer times 2 db 0
    err_msg_zero db "Error '-': Cannot decrement zero", 10
    err_msg_zero_len equ $-err_msg_zero
    err_msg_max db "Error '+': Cannot increment more than 255", 10
    err_msg_max_len equ $-err_msg_max
    err_msg_last db "Error '>': already at the last cell", 10
    err_msg_last_len equ $-err_msg_last
    err_msg_first db "Error '<': already at the first cell", 10
    err_msg_first_len equ $-err_msg_first

section .text
    global _start

putChar:
    mov eax, 1
    mov edi, 1
    mov esi, buffer
    mov edx, 1
    syscall
    ret

getChar:
    mov eax, 0
    mov edi, 0
    mov esi, buffer
    mov edx, 2
    syscall
    ret

_start:
    mov ebx, array
    mov edx, 0"#,
            cell_count
        )
    }

    fn error_messages(&self) -> String {
        r#"
exit_program:
    mov rax, 60
    mov rdi, 0
    syscall
label_err_msg_zero:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_zero
    mov edx, err_msg_zero_len
    syscall
    jmp exit_program
label_err_msg_max:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_max
    mov edx, err_msg_max_len
    syscall
    jmp exit_program
label_err_msg_last:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_last
    mov edx, err_msg_last_len
    syscall
    jmp exit_program
label_err_msg_first:
    mov eax, 1
    mov edi, 1
    mov esi, err_msg_first
    mov edx, err_msg_first_len
    syscall
    jmp exit_program"#
            .to_string()
    }

    fn shift_right(&self) -> String {
        format!(
            r#"
    ;; ---- Shift Right ----
    cmp edx, {}
    jz label_err_msg_last
    add ebx, 1
    add edx, 1"#,
            self.cell_count
        )
    }

    fn shift_left(&self) -> String {
        r#"
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, 1
    sub edx, 1"#
            .to_string()
    }

    fn increment(&self, i: u8) -> String {
        format!(
            r#"
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, {}
    mov byte [ebx], al"#,
            i
        )
    }

    fn decrement(&self, i: u8) -> String {
        format!(
            r#"
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, {}
    mov byte [ebx], al"#,
            i
        )
    }

    fn loop_start(&self) -> String {
        format!(
            r#"
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_{}:
    push rcx"#,
            self.loop_idx
        )
    }

    fn loop_end(&self) -> String {
        format!(
            r#"
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_{}"#,
            self.loop_idx
        )
    }

    fn output(&self) -> String {
        r#"
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar"#
            .to_string()
    }

    fn input(&self) -> String {
        r#"
    ;; ---- Get Char ----
    call getChar
    mov al, byte [buffer]
    mov byte [ebx], al"#
            .to_string()
    }
}
