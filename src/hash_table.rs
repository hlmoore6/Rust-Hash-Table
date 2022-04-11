use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct HashTable<K: Hash + PartialEq + std::clone::Clone, V: std::clone::Clone> {
    data: Vec<Option<(K, V)>>
}

impl<K: Hash + PartialEq + std::clone::Clone + std::fmt::Display, V: std::clone::Clone> HashTable<K, V> {

    pub fn new(size: usize) -> Self {
        let mut ht = HashTable {
            data: Vec::with_capacity(size)
        };

        ht.data.resize(size, None);

        ht
    }

    pub fn insert(&mut self, key: K, value: V) {
        
        let mut hashed_key = self.calculate_hash(&key);
        let data_capacity = self.data.capacity();

        let mut searched_indexes: usize = 0;
        while let Some(Some(entry)) = self.data.get(hashed_key) {
            if entry.0 == key {
                self.data[hashed_key] = Some((key, value));
                return;
            }
            if searched_indexes > data_capacity {
                panic!("There was no available index found!");
            }

            hashed_key += 1;
            searched_indexes += 1;
            hashed_key %= self.data.capacity();
        }

        self.data[hashed_key] = Some((key, value));
    }

    pub fn get(&self, key: K) -> Option<V> {
        let mut hashed_key = self.calculate_hash(&key);
        let data_capacity = self.data.capacity();

        while let Some(Some(entry)) = self.data.get(hashed_key) {
            if entry.0 == key {
                return Some(entry.1.clone());
            }

            hashed_key += 1;
            hashed_key %= data_capacity;
        }

        None
    }

    fn calculate_hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);

        let hashed_key = hasher.finish() as usize;

        hashed_key % self.data.capacity()
    }

    pub fn print_hash_table(&self) 
    where K: std::fmt::Display,
          V: std::fmt::Display {
        for item in self.data.iter() {
            if let Some(item) = item {
                println!("Key: {}    Value: {}", item.0, item.1);
            }
        }
    }
    
}
