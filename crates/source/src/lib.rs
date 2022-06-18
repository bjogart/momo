use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::Path;
use std::rc::Rc;

use la_arena::Arena;
use la_arena::Idx;
use query_base::QueryBase;

pub trait Queries<'qs>: QueryBase {
    fn src_map(self) -> &'qs SrcMap;

    fn src(self, input: Idx<Src>) -> Src {
        self.src_map().map[input].clone()
    }

    fn entry_src(self) -> Src {
        let src = self.src_map().entry;
        self.src_map().map[src].clone()
    }
}

pub struct SrcMap {
    entry: Idx<Src>,
    map: Arena<Src>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Src(Rc<String>);

impl SrcMap {
    pub fn new(entry: impl AsRef<Path>) -> Result<Self, io::Error> {
        fs::read_to_string(entry).map(|text| {
            let mut sources = Arena::new();
            let entry = sources.alloc(Src(Rc::new(text)));
            Self { entry, map: sources }
        })
    }
}

impl Src {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
