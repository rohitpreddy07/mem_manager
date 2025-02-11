use mem_manager::bmm::BitMemManager;
use rand::{thread_rng, Rng};
use std::ptr::null;
use std::time::{Duration, Instant};

fn main() {
    let dummy: *const u64 = null();
    let mut bit_man = BitMemManager::new(dummy, 256, 0);
    let mem_size = vec![8000, 16000, 32000];
    for num_pages in mem_size {
        bit_man.total_size = num_pages;
        BitMemManager::init_bitmap_rand(&mut bit_man);
        let mut i = 0;
        let mut bit_times: Vec<Duration> = Vec::new();
        let mut fail = 0;
        let mut success = 0;

        while i < 1000 {
            if fail == 1000 {
                println! {"Reached 1000 alloc fails"};
                break;
            }
            let mut rng = thread_rng();
            let pages = rng.gen_range(3..=12);
            let start_alloc = Instant::now();

            let mem = BitMemManager::allocate_page(&mut bit_man);
            let duration_alloc = start_alloc.elapsed();

            if mem.is_none() {
                fail += 1;
                continue;
            } else {
                success += 1;
            }

            if success == 10 {
                BitMemManager::deallocate_pages(&mut bit_man, mem.unwrap(), 1);
            }

            bit_times.push(duration_alloc);

            i += 1;
            fail = 0;
        }

        let bit_sum: Duration = bit_times.iter().sum();

        let avg_bit_times = bit_sum / bit_times.len() as u32;

        println!("For memory size = {}", num_pages);
        println!("Number of successful operations = {}", bit_times.len());
        println!("Bitmap avg allocation time: {:?}", avg_bit_times);
    }
}
