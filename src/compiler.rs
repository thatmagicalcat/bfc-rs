use std::{fs, io::Write};

use super::lexer::{Token, Tokens, lex};
use super::DEFAULT_CELL_COUNT;

#[derive(Debug, Clone)]
pub struct Compiler {
    tokens: Vec<(Token, u32)>,
    cell_count: usize,
    loop_idx: usize, // required to give names to the loop labels
    compiled_source: String,
    compiled: bool
}

impl Compiler {
    pub fn new(code: &str) -> Self {
        Self {
            tokens: Self::preprocess_tokens(lex(code.to_string())),
            compiled_source: Self::init(DEFAULT_CELL_COUNT),
            cell_count: DEFAULT_CELL_COUNT,
            loop_idx: 0,
            compiled: false
        }
    }

    pub fn with_cells(code: &str, cell_count: usize) -> Self {
        Self {
            tokens: Self::preprocess_tokens(lex(code.to_string())),
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
        for (token, occur) in self.tokens.clone().iter() {
            match token {
                Token::Increment  => self.increment(*occur as u8),
                Token::Decrement  => self.decrement(*occur as u8),
                Token::Input      => (0..*occur).for_each(|_| self.input()),
                Token::Output     => (0..*occur).for_each(|_| self.output()),
                Token::ShiftLeft  => self.shift_left(*occur),
                Token::ShiftRight => self.shift_right(*occur),

                Token::LoopStart  => { self.loop_idx += 1; self.loop_start() },
                Token::LoopEnd    => self.loop_end(),

                _ => unreachable!(),
            }
        }

        self.error_messages();
    }

    fn preprocess_tokens(tokens: Tokens) -> Vec<(Token, u32)> {
        let mut v = Vec::new();

        if !tokens.0.is_empty() {
            v.push((*tokens.0.first().unwrap(), 1));

            for itm in tokens.0.into_iter().skip(1) {
                let last = v.last_mut().unwrap();

                if itm == last.0 {
                    last.1 += 1;
                    continue;
                }

                v.push((itm, 1));
            }
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

    fn error_messages(&mut self) {
        self.compiled_source += r#"
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
    jmp exit_program"#;
    }

    fn shift_right(&mut self, i: u32) {
        self.compiled_source += format!(
            r#"
    ;; ---- Shift Right ----
    cmp edx, {0}
    jz label_err_msg_last
    add ebx, {1}
    add edx, {1}"#,
            self.cell_count, i
        ).as_str();
    }

    fn shift_left(&mut self, i: u32) {
        self.compiled_source += format!(
            r#"
    ;; ---- Shift Left ----
    cmp edx, 0
    jz label_err_msg_first
    sub ebx, {0}
    sub edx, {0}"#, i).as_str();
    }

    fn increment(&mut self, i: u8) {
        self.compiled_source += format!(
            r#"
    ;; ---- Increment ----
    mov al, byte [ebx]
    cmp al, 255
    jz label_err_msg_max
    add al, {}
    mov byte [ebx], al"#,
            i
        ).as_str();
    }

    fn decrement(&mut self, i: u8) {
        self.compiled_source += format!(
            r#"
    ;; ---- Decrement ----
    mov al, byte [ebx]
    cmp al, 0
    jz label_err_msg_max
    sub al, {}
    mov byte [ebx], al"#,
            i
        ).as_str();
    }

    fn loop_start(&mut self) {
        self.compiled_source += format!(
            r#"
    ;; ---- Loop Start ----
    movzx ecx, byte [ebx]
init_loop_{}:
    push rcx"#,
            self.loop_idx
        ).as_str();
    }

    fn loop_end(&mut self) {
        self.compiled_source += format!(r#"
   ;; ---- Loop End ----
    pop rcx
    dec rcx
    cmp rcx, 0
    jnz init_loop_{}"#,
            self.loop_idx
        ).as_str();
    }

    fn output(&mut self) {
        self.compiled_source += r#"
    ;; ---- Print Char ----
    mov al, byte [ebx]
    mov byte [buffer], al
    call putChar"#;
    }

    fn input(&mut self) {
        self.compiled_source += r#"
    ;; ---- Get Char ----
    call getChar
    mov al, byte [buffer]
    mov byte [ebx], al"#;
    }
}
