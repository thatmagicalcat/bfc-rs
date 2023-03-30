#![allow(unused)]

pub const DEFAULT_CELL_COUNT: usize = 2_000;

pub mod compiler;
pub mod interpreter;
pub mod lexer;

pub(crate) fn report_lex_error(code: &str, index: usize) {
    let line_number = code[..index].lines().count();
    let line_start = code[..index].rfind('\n').map_or(0, |i| i + 1);
    let line = &code[line_start..];
    let column_number = line.chars().take_while(|&c| c != '\n').count();

    let error_message = format!("Error at line {line_number}, column {column_number}: Invalid brainfuck instruction");
    eprintln!("{}", error_message);
}

pub(crate) fn report_error(msg: &str, code: &str, index: usize) {
    let line_number = code[..index].lines().count();
    let line_start = code[..index].rfind('\n').map_or(0, |i| i + 1);
    let line = &code[line_start..];
    let column_number = line.chars().take_while(|&c| c != '\n').count();

    let error_message = format!("Error at line {line_number}, column {column_number}: {msg}");
    eprintln!("{}", error_message);
}
