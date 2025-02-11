use rand::{thread_rng, Rng};

pub struct BitMemManager {
    pub bitmap: Vec<u64>,
    pub base_address: *const u64,
    pub total_size: usize,
    pub page_size: usize,
}

impl BitMemManager {
    pub fn new(base_address: *const u64, total_size: usize, page_size: usize) -> BitMemManager {
        let capacity = (total_size + 63) / 64;
        let bitmap = vec![0u64; capacity];

        BitMemManager {
            bitmap,
            base_address,
            total_size,
            page_size,
        }
    }

    pub fn init_bitmap_rand(&mut self) {
        let mut rng = thread_rng();
        let capacity = (self.total_size + 63) / 64;

        for _ in 0..capacity {
            let word: u64 = rng.gen_range(0..=0xFFFF_FFFF_FFFF_FFFF);
            self.bitmap.push(word);
        }
    }

    fn bitscan_zero_counter(bitmap: &Vec<u64>, num_pages: usize) -> Option<usize> {
        let mut i = 0;
        while i < bitmap.len() * 64 {
            let word_index = i / 64;
            let mut word = bitmap[word_index];
            let leading_zeros = word.leading_zeros() as usize;

            let mut bit_position = i % 64;

            if leading_zeros != 0 {
                if leading_zeros >= num_pages {
                    return Some(64 * (word_index + 1) - num_pages);
                }

                if word_index != bitmap.len() - 1 {
                    let next_trailing_zeros = bitmap[word_index + 1].trailing_zeros() as usize;

                    if leading_zeros + next_trailing_zeros >= num_pages {
                        return Some(next_trailing_zeros + (word_index + 1) * 64 - num_pages);
                    }
                }
            }

            while bit_position < 64 {
                let trailing_zeros = word.trailing_zeros() as usize;
                let position_in_bitmap = word_index * 64 + trailing_zeros + bit_position;

                if trailing_zeros >= num_pages {
                    return Some(position_in_bitmap - num_pages);
                }

                word >>= trailing_zeros + 1;
                bit_position += trailing_zeros + 1;

                if word == 0 && bit_position != 64 {
                    bit_position += leading_zeros;
                }
            }

            i = (word_index + 1) * 64;
        }

        None
    }

    fn bitscan_zero_counter_single_alloc(bitmap: &Vec<u64>, num_pages: usize) -> Option<usize> {
        let mut i = 0;
        while i < bitmap.len() * 64 {
            let word_index = i / 64;
            let mut word = bitmap[word_index];
            let leading_zeros = word.leading_zeros() as usize;

            let mut bit_position = i % 64;

            if leading_zeros != 0 {
                return Some(64 * (word_index + 1) - num_pages);
            }

            while bit_position < 64 {
                let trailing_zeros = word.trailing_zeros() as usize;
                let position_in_bitmap = word_index * 64 + trailing_zeros + bit_position;

                if trailing_zeros != 0 {
                    return Some(position_in_bitmap - num_pages);
                }
                word >>= trailing_zeros + 1;
                bit_position += trailing_zeros + 1;
            }

            i = (word_index + 1) * 64;
        }

        None
    }

    pub fn allocate_page(&mut self) -> Option<*mut u64> {
        let found = Self::bitscan_zero_counter_single_alloc(&self.bitmap, 1);

        if let Some(start) = found {
            for i in start..start + 1 {
                let word = i / 64;
                let bit = i % 64;
                self.bitmap[word] |= 1 << bit;
            }

            let base_ptr = self.bitmap.as_ptr();
            let bit_offset = start;
            let ptr = (base_ptr as usize) + bit_offset;
            Some(ptr as *mut u64)
        } else {
            None
        }
    }

    pub fn allocate_pages(&mut self, num_pages: usize) -> Option<*mut u64> {
        let found = Self::bitscan_zero_counter(&self.bitmap, num_pages);

        if let Some(start) = found {
            for i in start..start + num_pages {
                let word = i / 64;
                let bit = i % 64;
                self.bitmap[word] |= 1 << bit;
            }

            let base_ptr = self.bitmap.as_ptr();
            let bit_offset = start;
            let ptr = (base_ptr as usize) + bit_offset;
            Some(ptr as *mut u64)
        } else {
            None
        }
    }

    pub fn deallocate_pages(&mut self, address: *mut u64, num_pages: usize) {
        let start = (address as usize) - (self.bitmap.as_ptr() as usize);
        for i in start..start + num_pages {
            let word = i / 64;
            let bit = i % 64;
            self.bitmap[word] &= !(1 << bit);
        }
    }
}
