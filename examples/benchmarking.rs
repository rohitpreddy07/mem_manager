use mem_manager::bmm::BitMemManager;
use mem_manager::hmm::HashMemManager;
use rand::{thread_rng, Rng};
use std::ptr::null;
use std::time::{Duration, Instant};

fn main() {
    let dummy: *const u64 = null();
    let mut bit_man = BitMemManager::new(dummy, 256, 0);
    let mut hash_man = HashMemManager::new(dummy, 256, 0);
    let mem_size = vec![8000, 16000, 320000];
    for num_pages in mem_size {
        bit_man.total_size = num_pages;
        hash_man.total_size = num_pages;
        BitMemManager::init_bitmap_rand(&mut bit_man);
        HashMemManager::bitmap_to_hashmap(&mut hash_man, &mut bit_man.bitmap);
        let mut i = 0;
        let mut bit_times: Vec<(Duration, Duration)> = Vec::new();
        let mut hash_times: Vec<(Duration, Duration)> = Vec::new();

        while i < 1000 {
            let mut rng = thread_rng();
            let pages = rng.gen_range(3..=12);
            let start_alloc = Instant::now();

            let mem = BitMemManager::allocate_pages(&mut bit_man, pages);
            let duration_alloc = start_alloc.elapsed();

            if mem.is_none() {
                continue;
            }

            let start_dealloc = Instant::now();
            BitMemManager::deallocate_pages(&mut bit_man, mem.unwrap(), pages);
            let duration_dealloc = start_dealloc.elapsed();

            bit_times.push((duration_alloc, duration_dealloc));

            let start_alloc = Instant::now();
            let index = HashMemManager::alloc_hashmap(&mut hash_man, num_pages);
            let duration_alloc = start_alloc.elapsed();

            if index.is_none() {
                bit_times.pop();
                continue;
            }

            let start_dealloc = Instant::now();
            HashMemManager::dealloc_hashmap(&mut hash_man, index.unwrap());
            let duration_dealloc = start_dealloc.elapsed();

            hash_times.push((duration_alloc, duration_dealloc));

            i += 1;
        }

        assert!(bit_times.len() == hash_times.len());

        let bit_sum = bit_times
            .iter()
            .fold((Duration::ZERO, Duration::ZERO), |acc, &(a, b)| {
                (acc.0 + a, acc.1 + b)
            });

        let hash_sum = hash_times
            .iter()
            .fold((Duration::ZERO, Duration::ZERO), |acc, &(a, b)| {
                (acc.0 + a, acc.1 + b)
            });

        let avg_bit_times = (bit_sum.0 / 1000, bit_sum.1 / 1000);
        let avg_hash_times = (hash_sum.0 / 1000, hash_sum.1 / 1000);

        println!("For num_pages = {}", num_pages);
        println!("Number of operations = {}", bit_times.len());
        println!("Bitmap avg allocation time: {:?}", avg_bit_times.0);
        println!("Bitmap avg deallocation time: {:?}", avg_bit_times.1);
        println!("HashMap avg allocation time: {:?}", avg_hash_times.0);
        println!("HashMap avg deallocation time: {:?}", avg_hash_times.1);
    }
}
