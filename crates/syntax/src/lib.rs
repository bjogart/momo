use smol_str::SmolStr;

pub struct Token {
    kind: SyntaxKind,
    s: SmolStr,
}

impl Token {
    pub fn new(kind: SyntaxKind, s: String) -> Self {
        Self { kind, s: SmolStr::new(&s) }
    }
}

#[repr(u16)]
pub enum SyntaxKind {
    DUMMY,
}
