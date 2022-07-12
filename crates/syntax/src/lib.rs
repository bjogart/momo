use smol_str::SmolStr;

#[derive(Clone, Debug)]
pub struct Token {
    kind: SyntaxKind,
    s: SmolStr,
}

impl Token {
    pub fn new(kind: SyntaxKind, s: &str) -> Self {
        Self { kind, s: SmolStr::new(s) }
    }

    pub fn len(&self) -> usize  { self.s.len()}

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

#[repr(u16)]
pub enum SyntaxKind {
    DUMMY,
}
