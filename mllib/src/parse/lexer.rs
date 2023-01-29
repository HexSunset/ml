use super::Token::{self, *};

pub struct Lexer<'a> {
    input: &'a str,
    prev_token: Option<Token>,

}

impl<'a> Lexer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Self {
            input: input,
            prev_token: None,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.input = self.input.trim();

        if self.input.len() == 0 {
            return None;
        }

        let mut range = self.input.chars();
        let first = range.next().unwrap();
        match first {
            '(' => {
                self.input = &self.input[1..];

                self.prev_token = Some(OpenParen);
                return Some(OpenParen);
            },
            ')' => {
                self.input = &self.input[1..];

                self.prev_token = Some(CloseParen);
                return Some(CloseParen);
            },
            _ => {},
        }

        if first.is_ascii_digit() {
            let mut number_str: String = String::new();
            number_str.push(first);
            for c in range.take_while(|x| x.is_ascii_digit()) {
                number_str.push(c);
            }

            let number: i64 = number_str.parse().unwrap();

            self.input = &self.input[number_str.len()..];

            self.prev_token = Some(Number(0));
            return Some(Number(number));
        }

        let mut sym: String = String::new();
        sym.push(first);
        for c in range.take_while(|x| !x.is_ascii_whitespace()) {
            sym.push(c);
        }

        self.input = &self.input[sym.len()..];

        if self.prev_token == Some(OpenParen) {
            return Some(Function(sym));
        }

        return Some(Symbol(sym));
    }
}
