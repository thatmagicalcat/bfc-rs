use logos::{Lexer, Logos};

use super::report_lex_error;

#[derive(Debug, Clone)]
pub(crate) struct Tokens(pub(crate) Vec<Token>);

pub(crate) fn lex(input: String) -> Tokens {
    let mut l = Token::lexer(input.as_str()).into_iter().collect::<Vec<_>>();

    let mut error_idx = Vec::new();

    for (idx, itm) in l.iter().enumerate() {
        if *itm == Token::Error {
            error_idx.push(idx);
        }
    }

    (!error_idx.is_empty()).then(|| error_idx.iter().for_each(|e| report_lex_error(&input, *e)));
    (!error_idx.is_empty()).then(|| std::process::exit(1));

    Tokens(
        l.iter()
            .filter(|&&x| x != Token::Skip)
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub(crate) enum Token {
    #[token("<")]
    ShiftLeft,

    #[token(">")]
    ShiftRight,

    #[token("[")]
    LoopStart,

    #[token("]")]
    LoopEnd,

    #[token("+")]
    Increment,

    #[token("-")]
    Decrement,

    #[token(".")]
    Output,

    #[token(",")]
    Input,

    #[regex(r"[ \t\n\f\r]+")]
    Skip,

    #[error]
    Error,
}
