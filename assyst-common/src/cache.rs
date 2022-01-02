use std::collections::HashMap;
use std::hash::Hash;

pub struct Cache<T, U> {
    pub cache: HashMap<T, U>,
    pub limit: usize,
}
impl<T: Hash + Eq + Clone, U> Cache<T, U> {
    pub fn new(limit: usize) -> Self {
        if limit == 0 {
            panic!("limit must be >0")
        };
        Cache {
            cache: HashMap::new(),
            limit,
        }
    }

    pub fn insert(&mut self, key: T, value: U) {
        let limit = self.limit.clone();
        let len = self.cache.len().clone();

        if len == limit {
            let fst = self.cache.iter().next().unwrap().0.clone();
            self.cache.remove(&fst);
        }

        self.cache.insert(key, value);
    }

    pub fn get(&self, key: &T) -> Option<&U> {
        self.cache.get(key)
    }

    pub fn get_mut(&mut self, key: &T) -> Option<&mut U> {
        self.cache.get_mut(key)
    }
}
