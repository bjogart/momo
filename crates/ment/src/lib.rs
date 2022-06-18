use name::Def;
use query_base::QueryBase;

pub trait Queries<'qs>: QueryBase + mair::Queries<'qs> {
    fn ment_func(self, def: Def) -> Func {
        let mair = self.mair_func(def);
        lower_func(mair)
    }
}

pub struct Func;

fn lower_func(func: mair::Func) -> Func {
    todo!()
}
