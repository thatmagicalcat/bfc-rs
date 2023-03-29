use logos::{Logos, Lexer};

#[derive(Debug, Clone)]
pub struct Tokens(pub(crate) Vec<Token>);

pub fn lex(input: String) -> Tokens {
    let mut l = Token::lexer(input.as_str()).into_iter().collect::<Vec<_>>();

    let mut error_idx = Vec::new();

    for (idx, itm) in l.iter().enumerate() {
        if *itm == Token::Error {
            error_idx.push(idx);
        }
    }

    (!error_idx.is_empty()).then(|| report_error(error_idx, &input));

    Tokens(l.iter().filter(|&&x| x != Token::Skip).map(|x| *x).collect::<Vec<_>>())
}

fn report_error(_idx: Vec<usize>, _input: &String) {
    unimplemented!()
}

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
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
    Error
}
