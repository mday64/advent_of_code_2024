use std::collections::{HashMap,HashSet};

pub trait HashPop<T> {
    fn pop(&mut self) -> Option<T>;
}

impl<V, S> HashPop<V> for HashSet<V, S>
where
    V: std::cmp::Eq,
    V: std::hash::Hash,
    V: Clone,
    S: std::hash::BuildHasher,
{
    fn pop(&mut self) -> Option<V> {
        if self.is_empty() {
            None
        } else {
            let v = self.iter().next().unwrap().clone();
            self.remove(&v);
            Some(v)
        }
    }
}

impl<K,V,S> HashPop<(K,V)> for HashMap<K, V, S>
where
    K: std::cmp::Eq,
    K: std::hash::Hash,
    K: Clone,
    S: std::hash::BuildHasher
{
    fn pop(&mut self) -> Option<(K,V)> {
        if self.is_empty() {
            None
        } else {
            let k = self.keys().next().unwrap().clone();
            let (k, v) = self.remove_entry(&k).unwrap();
            Some((k, v))
        }
    }
}
