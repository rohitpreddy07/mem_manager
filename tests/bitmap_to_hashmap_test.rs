#[cfg(test)]

    mod tests {
        use mem_manager::bmm::BitMemManager;
        use mem_manager::hmm::HashMemManager;
        use std::ptr::null;
        
        #[test]
        fn bitmap_to_hashmap() {
            let dummy: *const u64 = null();
            let mut bit_man = BitMemManager::new(dummy, 256, 0);
            let mut hash_man = HashMemManager::new(dummy, 256, 0);
            BitMemManager::init_bitmap_rand(&mut bit_man);
        
            for word in &bit_man.bitmap {
                println!("{:064b}", word);
            }
        
            HashMemManager::bitmap_to_hashmap(&mut hash_man, &bit_man.bitmap);
            println!("{:?}", hash_man.map);
        }

    }

