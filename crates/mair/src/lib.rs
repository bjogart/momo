mod lower;

use name::Def;
use query_base::QueryBase;

pub trait Queries<'qs>: QueryBase + fir::Queries<'qs> + name::Queries<'qs> {
    fn mair_func(self, def: Def) -> Func {
        let fir = self.fir_func(self.fir_id_for_def(def));
        lower::func(fir)
    }
}

pub struct Func;
