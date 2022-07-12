use itertools::Itertools;
use parse::Builder;
use query_base::QueryBase;
use source::Src;
use syntax::SyntaxKind;
use syntax::Token;

pub trait Queries: QueryBase + lexer::Queries + parse::Queries {
    fn tree(self, src: Src) -> Mod {
        let (lex_errors, tokens): (Vec<Token>, Vec<Token>) =
            self.tokens(src).into_iter().partition(|t| t.kind().is_err());

        if !lex_errors.is_empty() {
            panic!("lexing failed: {lex_errors:?}");
        }

        let tokens = tokens.into_iter().map(filter_tag_parse_token).collect_vec();
        let mut tokens_for_parser = Vec::with_capacity(tokens.len());
        let mut pos = 0;
        for (t, keep) in &tokens {
            let len = t.len();
            match keep {
                true => tokens_for_parser.push((pos, t.kind(), pos + len)),
                false => {}
            }
            pos += t.len();
        }

        let mut builder = TreeBuilder::new(tokens);
        self.parse(&mut builder, tokens_for_parser);
        let (tree, errors) = builder.build();

        if !errors.is_empty() {
            panic!("parsing failed: {errors:?}");
        }

        tree
    }
}

pub struct Mod;

#[derive(Debug)]
pub struct Error;

fn filter_tag_parse_token(t: Token) -> (Token, bool) {
     let keep = !matches!(t.kind(), SyntaxKind::WS);
    (t, keep)
}

pub struct TreeBuilder {
    tokens: Vec<(Token, bool)>,
}

impl TreeBuilder {
    pub fn new(tokens: Vec<(Token, bool)>) -> Self {
        Self { tokens }
    }
}

impl parse::Builder for TreeBuilder {
    type Out = Mod;
    type Error = Error;

    fn start(&mut self, kind: SyntaxKind) {
        todo!()
    }
    fn end(&mut self) {
        todo!()
    }
    fn token(&mut self) {
        todo!()
    }
    fn error(&mut self) {
        todo!()
    }
    fn build(self) -> (Self::Out, Vec<Self::Error>) {
        todo!()
    }
}
