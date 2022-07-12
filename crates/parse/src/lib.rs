use query_base::QueryBase;
use syntax::SyntaxKind;

pub trait Queries: QueryBase {
    fn parse<B: Builder>(self, builder: &mut B, tokens: Vec<(usize, SyntaxKind, usize)>) {
    }
}

pub trait Builder {
    type Out;
    type Error;

    fn start(&mut self, kind: SyntaxKind);
    fn end(&mut self);
    fn token(&mut self);
    fn error(&mut self);
    fn build(self) -> (Self::Out, Vec<Self::Error>);
}
