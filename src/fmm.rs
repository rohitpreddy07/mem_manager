use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

pub struct FreeMemManager {
    pub free_map: BTreeMap<u32, u32>,
    pub base_address: *const u64,
    pub total_size: u32,
    pub page_size: u32,
}

impl FreeMemManager {
    pub fn new(base_address: *const u64, total_size: u32, page_size: u32) -> FreeMemManager {
        let mut free_map: BTreeMap<u32, u32> = BTreeMap::new();
        free_map.insert(0, total_size / page_size);

        FreeMemManager {
            free_map,
            base_address,
            total_size,
            page_size,
        }
    }

    pub fn alloc_freemap(&mut self, size: u32) -> Option<*const u64> {
        let num_pages = (size + self.page_size - 1) / self.page_size;

        let mut alloc_address = None;

        for (&index, &free_pages) in self.free_map.iter() {
            if free_pages >= num_pages {
                let new_index = index + num_pages;
                self.free_map.remove(&index);

                if free_pages > num_pages {
                    self.free_map.insert(new_index, free_pages - num_pages);
                }

                alloc_address = Some(
                    (self.base_address as usize + (index * self.page_size) as usize) as *const u64,
                );
                break;
            }
        }

        alloc_address
    }

    pub fn dealloc_freemap(&mut self, address: *const u64, size: u32) {
        let index =
            ((address as usize - self.base_address as usize) / self.page_size as usize) as u32;
        let num_pages = (size + self.page_size - 1) / self.page_size;

        let mut merge_left = false;
        let mut merge_right = false;
        let mut left_index = 0;
        let mut left_pages = 0;
        let mut right_index = 0;
        let mut right_pages = 0;

        if index > 0 {
            for (&free_index, &free_pages) in self.free_map.iter() {
                if index == free_index + free_pages {
                    merge_left = true;
                    left_index = index - free_pages;
                    left_pages = free_pages;
                }
            }
        }

        if let Some(&next_pages) = self.free_map.get(&(index + num_pages)) {
            merge_right = true;
            right_index = index + num_pages;
            right_pages = next_pages;
        }

        match (merge_left, merge_right) {
            (true, true) => {
                self.free_map.remove(&left_index);
                self.free_map.remove(&right_index);
                self.free_map
                    .insert(left_index, left_pages + num_pages + right_pages);
            }
            (true, false) => {
                self.free_map.remove(&left_index);
                self.free_map.insert(left_index, left_pages + num_pages);
            }
            (false, true) => {
                self.free_map.remove(&right_index);
                self.free_map.insert(index, num_pages + right_pages);
            }
            (false, false) => {
                self.free_map.insert(index, num_pages);
            }
        }
    }
}
