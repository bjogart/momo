use query_base::QueryBase;
use source::Src;

pub trait Queries<'qs>: QueryBase + source::Queries<'qs> + fir::Queries<'qs> {
    const ENTRY_FUNC_NAME: &'static str = "main";

    fn package_graph(self) -> &'qs PackGraph;

    fn entry_func(self) -> Def {
        let entry_mod = self.fir_module(self.fir_for_src(self.entry_src()));
        let func = entry_mod.items().find(|it| it.name() == Self::ENTRY_FUNC_NAME).unwrap();
        self.def(func.id())
    }

    fn fir_id_for_def(self, def: Def) -> fir::Id {
        todo!()
    }

    fn def(self, id: fir::Id) -> Def {
        todo!()
    }
}

pub struct PackGraph;

pub struct Def(usize);

struct Path;

impl PackGraph {
    fn src_path(&self, src: Src) -> Option<Path> {
        todo!()
    }
}
