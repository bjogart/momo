use query_base::QueryBase;
use source::Src;

pub trait Queries: QueryBase + parse::Queries {
    fn tree(self, src: Src) -> Mod {
        let (parse, errors) = self.parse(src, TreeBuilder::default());
        todo!()
    }
}

pub struct Mod;

#[derive(Default)]
pub struct TreeBuilder {}

impl parse::Builder for TreeBuilder {
    type Out = ((), Vec<parse::Error>);
}
