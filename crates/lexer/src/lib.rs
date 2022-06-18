use query_base::QueryBase;
use source::Src;
use syntax::Token;

pub trait Queries: QueryBase {
    fn tokens(src: Src) -> Vec<Token> {
        todo!()
    }
}
