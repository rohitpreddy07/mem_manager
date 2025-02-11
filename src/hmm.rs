use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub struct HashMemManager {
    pub map: HashMap<usize, usize>,
    pub base_address: *const u64,
    pub total_size: usize,
    pub page_size: usize,
}

impl HashMemManager {
    pub fn new(base_address: *const u64, total_size: usize, page_size: usize) -> HashMemManager {
        let map: HashMap<usize, usize> = HashMap::new();
        HashMemManager {
            map,
            base_address,
            total_size,
            page_size,
        }
    }

    pub fn rand_hashmap_init(&mut self, num_entries: usize) {
        let mut rng = thread_rng();
        let mut map = HashMap::new();

        if map.is_empty() {
            let random_key = rng.gen_range(0..self.total_size);
            let random_value = rng.gen_range(3..=12);
            map.insert(random_key, random_value);
        }

        while map.len() < num_entries {
            let random_key = rng.gen_range(0..self.total_size);
            let random_value = rng.gen_range(3..=12);
            let entry_start = random_key;
            let entry_end = entry_start + random_value;

            let mut overlap = false;
            for (&start, &value) in &map {
                let end = start + value;
                if (entry_start < end) && (entry_end > start) {
                    overlap = true;
                    break;
                }
            }

            if !overlap && entry_end <= self.total_size {
                map.insert(random_key, random_value);
            }
        }

        self.map = map;
    }

    pub fn alloc_hashmap(&mut self, num_pages: usize) -> Option<usize> {
        if self.map.is_empty() {
            self.map.insert(0, num_pages);
            return Some(0);
        }

        let mut keys: Vec<_> = self.map.keys().cloned().collect();
        keys.sort();

        for i in 0..keys.len() - 1 {
            let start = keys[i];
            let end = self.map[&start] + start;

            let next_start = keys[i + 1];

            if next_start > end && next_start - end >= num_pages {
                self.map.insert(end, num_pages);
                return Some(end);
            }
        }

        let last_start = *keys.last().unwrap();
        let last_end = last_start + self.map[&last_start];
        if self.total_size - last_end - 1 >= num_pages {
            self.map.insert(last_end - 1, num_pages);
            return Some(last_end - 1);
        }

        None
    }

    pub fn dealloc_hashmap(&mut self, address: usize) {
        self.map.remove(&address);
    }

    pub fn bitmap_to_hashmap(&mut self, bitmap: &Vec<u64>) {
        let total_bits = bitmap.len() * 64;
        let mut consecutive_ones = 0;
        let mut start = None;

        for i in 0..total_bits {
            let word = i / 64;
            let bit = i % 64;

            if bitmap[word] & (1 << bit) != 0 {
                if consecutive_ones == 0 {
                    start = Some(i);
                }
                consecutive_ones += 1;
            } else {
                if let Some(start_index) = start {
                    self.map.insert(start_index, consecutive_ones);
                    consecutive_ones = 0;
                    start = None;
                }
            }
        }

        if let Some(start_index) = start {
            self.map.insert(start_index, consecutive_ones);
        }
    }
}
