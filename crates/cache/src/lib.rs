use core::hash::Hash;

use hashbrown::hash_map::RawEntryMut;
use hashbrown::HashMap;

pub struct Cache<I, O> {
    cache: HashMap<I, O>,
}

impl<I, O> Cache<I, O> {
    pub fn get(&mut self, input: I, f: impl FnOnce(&I) -> O) -> O
    where
        I: Eq + Hash,
        O: Clone,
    {
        match self.cache.raw_entry_mut().from_key(&input) {
            RawEntryMut::Occupied(entry) => entry.get().clone(),
            RawEntryMut::Vacant(entry) => {
                let out = f(&input);
                entry.insert(input, out).1.clone()
            }
        }
    }
}

impl<I, O> Default for Cache<I, O> {
    fn default() -> Self {
        Self { cache: HashMap::default() }
    }
}
