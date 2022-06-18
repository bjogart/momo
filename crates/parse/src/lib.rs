use query_base::QueryBase;
use source::Src;

pub trait Queries: QueryBase {
    fn parse<B: Builder>(self, src: Src, builder: B) -> B::Out {
        todo!()
    }
}

pub enum Error {}

pub trait Builder {
    type Out;
}
