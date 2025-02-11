#[cfg(test)]

mod tests {
    use mem_manager::bmm::BitMemManager;
    use rand::{thread_rng, Rng};
    use std::ptr::null;

    #[test]

    fn bitmap_leading_test() {
        let dummy: *const u64 = null();
        let mut bit_man = BitMemManager::new(dummy, 256, 0);
    
        let mut rng = thread_rng();
    
        let index = rng.gen_range(0..3);
    
        BitMemManager::init_bitmap_rand(&mut bit_man);
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("warped bitmap");
        let length = bit_man.bitmap.len() * 64;
        let range = (56 * (index + 1) + 8 * index)..(64 * (index + 1));
        for i in 0..length {
            if range.contains(&i) {
                let word = i / 64;
                let bit = i % 64;
                bit_man.bitmap[word] &= !(1 << bit);
            }
        }
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("base address of bitmap: {:?}", &bit_man.bitmap.as_ptr());
    
        let mem = BitMemManager::allocate_pages(&mut bit_man, 8).unwrap();
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("address to bit: {:?}", mem);
    
        BitMemManager::deallocate_pages(&mut bit_man, mem, 8);
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    }

    #[test]

    fn bitmap_spanning_test() {
        let dummy: *const u64 = null();
        let mut bit_man = BitMemManager::new(dummy, 256, 0);
    
        let mut rng = thread_rng();
    
        let index = rng.gen_range(0..3);
    
        BitMemManager::init_bitmap_rand(&mut bit_man);
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("warped bitmap");
        let length = bit_man.bitmap.len() * 64;
        let range = (59 * (index + 1) + 8 * index)..(68 * (index + 1));
        for i in 0..length {
            if range.contains(&i) {
                let word = i / 64;
                let bit = i % 64;
                bit_man.bitmap[word] &= !(1 << bit);
            }
        }
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("base address of bitmap: {:?}", &bit_man.bitmap.as_ptr());
    
        let mem = BitMemManager::allocate_pages(&mut bit_man, 8).unwrap();
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    
        println!("address to bit: {:?}", mem);
    
        BitMemManager::deallocate_pages(&mut bit_man, mem, 8);
    
        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    }

    #[test]
    fn bitmap_test() {
        let dummy: *const u64 = null();
        let mut bit_man = BitMemManager::new(dummy, 256, 0);

        BitMemManager::init_bitmap_rand(&mut bit_man);

        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }

        println!("base address of bitmap: {:?}", &bit_man.bitmap.as_ptr());

        let mem = BitMemManager::allocate_pages(&mut bit_man, 8).expect("Alloc Failed");

        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }

        println!("address to bit: {:?}", mem);

        BitMemManager::deallocate_pages(&mut bit_man, mem, 8);

        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }
    }

    #[test]
    fn empty_bitmap_test() {
        let dummy: *const u64 = null();
        let mut bit_man = BitMemManager::new(dummy, 256, 0);

        for word in &bit_man.bitmap {
            println!("{:064b}", word);
        }

        for i in 0..bit_man.total_size {
            let mem = BitMemManager::allocate_page(&mut bit_man);

            if mem.is_none() {
                println!("Alloc Failed");
                for word in &bit_man.bitmap {
                    println!("{:064b}", word);
                }
            }
        }
    }
}
