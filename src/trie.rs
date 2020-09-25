use crate::reference::Reference;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Serialize, Deserialize)]
pub struct Trie<K: Eq + Hash, V: Eq + Hash> {
    pub val: HashMap<V, HashSet<Reference>>,
    pub next: HashMap<K, Trie<K, V>>,
}

impl<K: Eq + Hash, V: Eq + Hash> Trie<K, V> {
    pub fn new() -> Self {
        Trie {
            val: HashMap::new(),
            next: HashMap::new(),
        }
    }
}
