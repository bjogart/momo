use smol_str::SmolStr;

#[derive(Clone, Debug)]
pub struct Token {
    kind: SyntaxKind,
    s: SmolStr,
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    DUMMY,
    ERR_UNEXPECTED_CHAR,
    LPAR,
    RPAR,
    COLON,
    EQ,
    COMMA,
    IDENT,
    PATH,
    INT,
    WS,
    NL,
    Ty,
}

impl Token {
    pub fn new(kind: SyntaxKind, s: &str) -> Self {
        Self { kind, s: SmolStr::new(s) }
    }

    pub fn len(&self) -> usize {
        self.s.len()
    }

    pub fn kind(&self) -> SyntaxKind {
        self.kind
    }
}

impl SyntaxKind {
    pub fn is_err(self) -> bool {
        matches!(self, Self::DUMMY | Self::ERR_UNEXPECTED_CHAR)
    }
}
