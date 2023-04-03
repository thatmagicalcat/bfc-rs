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

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Token {
    ShiftLeft,
    ShiftRight,
    LoopStart,
    LoopEnd,
    Increment,
    Decrement,
    Output,
    Input,
    Skip,
    Error,
}

impl Token {
    pub fn lexer(input: &str) -> Vec<Self> {
        use Token::*;

        let mut tokens = Vec::new();

        for ch in input.chars() {
            let t = match ch {
                '<' => ShiftLeft,
                '>' => ShiftRight,
                '+' => Increment,
                '-' => Decrement,
                '[' => LoopStart,
                ']' => LoopEnd,
                '.' => Output,
                ',' => Input,
                x if x.is_whitespace() => Skip,
                _ => Error,
            };

            (t != Skip).then(|| tokens.push(t));
        }

        tokens
    }
}
