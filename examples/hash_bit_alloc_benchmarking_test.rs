use mem_manager::bmm::BitMemManager;
use mem_manager::hmm::HashMemManager;
use rand::{thread_rng, Rng};
use std::ptr::null;
use std::time::{Duration, Instant};

fn main() {
    let dummy: *const u64 = null();
    let mut bit_man = BitMemManager::new(dummy, 256, 0);
    let mut hash_man = HashMemManager::new(dummy, 256, 0);
    let mem_size = vec![8000, 16000, 32000];
    for num_pages in mem_size {
        bit_man.total_size = num_pages;
        hash_man.total_size = num_pages;
        BitMemManager::init_bitmap_rand(&mut bit_man);
        HashMemManager::bitmap_to_hashmap(&mut hash_man, &mut bit_man.bitmap);
        let mut i = 0;
        let mut bit_times: Vec<Duration> = Vec::new();
        let mut hash_times: Vec<Duration> = Vec::new();
        let mut fail = 0;

        while i < 1000 {
            if fail == 1000 {
                println! {"Reached 1000 alloc fails"};
                break;
            }
            let mut rng = thread_rng();
            let pages = rng.gen_range(3..=12);
            let start_alloc = Instant::now();

            let mem = BitMemManager::allocate_pages(&mut bit_man, pages);
            let duration_alloc = start_alloc.elapsed();

            if mem.is_none() {
                fail += 1;
                continue;
            }

            bit_times.push(duration_alloc);

            let start_alloc = Instant::now();
            let index = HashMemManager::alloc_hashmap(&mut hash_man, num_pages);
            let duration_alloc = start_alloc.elapsed();

            if index.is_none() {
                bit_times.pop();
                fail += 1;
                continue;
            }

            hash_times.push(duration_alloc);

            i += 1;
            fail = 0;
        }

        assert!(bit_times.len() == hash_times.len());

        let bit_sum: Duration = bit_times.iter().sum();

        let hash_sum: Duration = hash_times.iter().sum();

        let avg_bit_times = bit_sum / bit_times.len() as u32;
        let avg_hash_times = hash_sum / hash_times.len() as u32;

        println!("For memory size = {}", num_pages);
        println!("Number of successful operations = {}", bit_times.len());
        println!("Bitmap avg allocation time: {:?}", avg_bit_times);
        println!("HashMap avg allocation time: {:?}", avg_hash_times);
        println!(
            "Ratio (Bit_alloc / Hash_alloc): {:?}",
            avg_hash_times.as_nanos() as f64 / avg_bit_times.as_nanos() as f64
        );
    }
}
