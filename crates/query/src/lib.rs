use std::path::Path;

use config::Config;
use fir::FirMap;
use name::PackGraph;
use query_base::QueryBase;
use source::SrcMap;

#[derive(Clone, Copy)]
pub struct QueryCx<'qx> {
    state: &'qx State,
}

struct State {
    config: Config,
    src_map: SrcMap,
    fir_map: FirMap,
}

impl QueryCx<'_> {
    pub fn enter<R>(config: Config, f: impl FnOnce(QueryCx) -> R) -> R {
        let state = State::new(config);
        let qx = QueryCx { state: &state };
        f(qx)
    }
}

impl QueryBase for QueryCx<'_> {}

impl<'qx> source::Queries<'qx> for QueryCx<'qx> {
    fn src_map(self) -> &'qx source::SrcMap {
        &self.state.src_map
    }
}

impl<'qx> lexer::Queries for QueryCx<'qx> {}
impl<'qx> parse::Queries for QueryCx<'qx> {}

impl<'qx> tree::Queries for QueryCx<'qx> {}

impl<'qx> name::Queries<'qx> for QueryCx<'qx> {
    fn package_graph(self) -> &'qx PackGraph {
        todo!()
    }
}

impl<'qx> fir::Queries<'qx> for QueryCx<'qx> {
    fn fir_map(self) -> &'qx fir::FirMap {
        &self.state.fir_map
    }
}

impl<'qx> mair::Queries<'qx> for QueryCx<'qx> {}

impl<'qx> ment::Queries<'qx> for QueryCx<'qx> {}

impl<'qx> code::Queries<'qx> for QueryCx<'qx> {}

impl State {
    fn new(config: Config) -> Self {
        let src_map = SrcMap::new(config.entry()).unwrap();
        Self { config, src_map, fir_map: FirMap::default() }
    }
}
