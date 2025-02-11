

#[cfg(test)]

    mod tests {
        use mem_manager::hmm::HashMemManager;
        use std::ptr::null;
        
        #[test]
        fn hashmap_test() {
            let dummy: *const u64 = null();
            let mut hash_man = HashMemManager::new(dummy, 256, 0);

            HashMemManager::rand_hashmap_init(&mut hash_man, 15);

            println!("map = {:?}", hash_man.map);

            let mem = HashMemManager::alloc_hashmap(&mut hash_man, 5).unwrap();

            println!("allocated map = {:?}", hash_man.map);

            HashMemManager::dealloc_hashmap(&mut hash_man, mem);

            println!("deallocated map = {:?}", hash_man.map);
        }
    }