mod lower;

use std::borrow::BorrowMut;
use std::cell::RefCell;

use cache::Cache;
use la_arena::Arena;
use la_arena::Idx;
use query_base::QueryBase;
use source::Src;

pub trait Queries<'qs>: QueryBase + tree::Queries {
    fn fir_map(self) -> &'qs FirMap;

    fn fir_for_src(self, src: Src) -> Mod {
        self.fir_map().source_module_map.borrow_mut().get(src, |src| {
            let syntax = self.tree(src.clone());
            let module = lower::module(syntax);
            let idx = self.fir_map().modules.borrow_mut().alloc(module);
            Mod(idx)
        })
    }

    fn fir_module(self, module: Mod) -> &'qs ModData {
        todo!()
    }

    fn fir_func(self, id: Id) -> Func {
        todo!()
    }
}

#[derive(Default)]
pub struct FirMap {
    source_module_map: RefCell<Cache<Src, Mod>>,
    modules: RefCell<Arena<ModData>>,
}

pub enum Fir {}

#[derive(Clone, Copy)]
pub struct Id;

#[derive(Clone, Copy)]
pub struct Mod(Idx<ModData>);

pub struct ModData;

#[derive(Clone, Copy)]
pub struct Item;

pub struct Func;

impl ModData {
    pub fn items(&self) -> impl Iterator<Item = Item> {
        [todo!()].iter().copied()
    }
}

impl Item {
    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn id(&self) -> Id {
        todo!()
    }
}
