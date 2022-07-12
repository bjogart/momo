use lalrpop_util::lalrpop_mod;
use query_base::QueryBase;
use syntax::SyntaxKind;

lalrpop_mod!(parser);

pub trait Queries: QueryBase {
    fn parse<B: Builder>(self, builder: &mut B, tokens: Vec<(usize, SyntaxKind, usize)>) {
        match parser::FileParser::new().parse(builder, tokens.into_iter()) {
            Ok(_) => {}
            Err(err) => panic!("lalrpop parser error: `{err:?}`"),
        }
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
