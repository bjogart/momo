use std::str::Chars;

use itertools::Itertools;
use query_base::QueryBase;
use source::Src;
use syntax::SyntaxKind;
use syntax::Token;
use unicode_xid::UnicodeXID;

pub trait Queries: QueryBase {
    fn tokens(self, src: Src) -> Vec<Token> {
        let text = src.as_str();
        let pos = &mut 0;
        text.chars()
            .batching(|it| scan_next_token(text, pos, it))
            .collect_vec()
    }
}

trait LexChar {
    fn is_ident_start(&self) -> bool;
    fn is_ident_cont(&self) -> bool;
    fn is_ws(&self) -> bool;
    fn is_decimal_digit(&self) -> bool;
}

fn scan_next_token(text: &str, pos: &mut usize, it: &mut Chars<'_>) -> Option<Token> {
    eat(it).map(|(ch, mut len)| {
        let kind = match ch {
            ch if ch.is_ident_start() => {
                len += eat_while(it, LexChar::is_ident_cont);

                let n_consumed = eat_while(it, |&ch| ch.is_ident_cont() || ch == '/');
                if n_consumed > 0 {
                    len += n_consumed;
                    SyntaxKind::PATH
                } else {
                SyntaxKind::IDENT
                }
            }
            ch if ch.is_ws() => {
                len += eat_while(it, LexChar::is_ws);
                SyntaxKind::WS
            }
            ch if ch.is_decimal_digit() => {
                len += eat_while(it, LexChar::is_decimal_digit);
                SyntaxKind::INT
            }
            '\n' => {
                len += eat_while(it, |&ch| ch == '\n');
                SyntaxKind::NL
            }
            '=' => {
                    SyntaxKind::EQ
            '(' => SyntaxKind::LPAR,
            ')' => SyntaxKind::RPAR,
            ':' => SyntaxKind::COLON,
            ',' => SyntaxKind::COMMA,
            _ => SyntaxKind::ERR_UNEXPECTED_CHAR,
        let t = Token::new(kind, &text[*pos..*pos + len]);
        *pos += len;
        t
    })
            }

impl LexChar for char {
    fn is_ident_start(&self) -> bool {
        self.is_xid_start()
    }

    fn is_ident_cont(&self) -> bool {
        self.is_xid_continue()
    }

    fn is_ws(&self) -> bool {
        matches!(self, ' ')
    }

    fn is_decimal_digit(&self) -> bool {
        matches!(self, '0'..='9')
    }
}

fn eat(it: &mut Chars<'_>) -> Option<(char, usize)> {
    it.next().map(|ch| (ch, ch.len_utf8()))
}

fn eat_if(it: &mut Chars<'_>, pred: impl Fn(&char) -> bool) -> Option<usize> {
    match it.clone().next() {
        Some(next) if pred(&next) => eat(it).map(|(_, len)| len),
        _ => None,
    }
}

fn eat_while(it: &mut Chars<'_>, pred: impl Fn(&char) -> bool) -> usize {
    it.take_while_ref(pred).map(char::len_utf8).sum::<usize>()
}
