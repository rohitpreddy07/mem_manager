# Memory Management Module

## Overview
This Rust module provides two different memory management strategies for handling memory allocation and deallocation:

1. **BitMemManager**: Uses a bitmap-based approach to track memory pages efficiently.
2. **FreeMemManager**: Uses a `BTreeMap` to track free memory blocks, allowing more flexible allocations.

## Features
- **Efficient Page Tracking**: The bitmap-based method minimizes overhead for managing small allocations.
- **Flexible Memory Management**: The free map approach supports dynamic allocation and merging of free blocks.
- **Randomized Bitmap Initialization**: The bitmap manager can be initialized with random values for testing purposes.
- **Efficient Deallocation**: Both managers support merging of adjacent free blocks to reduce fragmentation.

## Dependencies
This module relies on:
- `rand` for random number generation (used in `init_bitmap_rand`).
- `std::collections::BTreeMap` for efficient free memory management in `FreeMemManager`.

## BitMemManager
### Usage
#### Initialization
```rust
let base_address: *const u64 = std::ptr::null();
let mut bit_manager = BitMemManager::new(base_address, 1024, 64);
```

#### Allocating a Page
```rust
if let Some(ptr) = bit_manager.allocate_page() {
    println!("Allocated page at: {:?}", ptr);
}
```

#### Allocating Multiple Pages
```rust
if let Some(ptr) = bit_manager.allocate_pages(4) {
    println!("Allocated 4 pages at: {:?}", ptr);
}
```

#### Deallocating Pages
```rust
if let Some(ptr) = bit_manager.allocate_pages(4) {
    bit_manager.deallocate_pages(ptr, 4);
    println!("Deallocated 4 pages at: {:?}", ptr);
}
```

## FreeMemManager
### Usage
#### Initialization
```rust
let base_address: *const u64 = std::ptr::null();
let mut free_manager = FreeMemManager::new(base_address, 1024, 64);
```

#### Allocating Memory
```rust
if let Some(ptr) = free_manager.alloc_freemap(128) {
    println!("Allocated 128 bytes at: {:?}", ptr);
}
```

#### Deallocating Memory
```rust
if let Some(ptr) = free_manager.alloc_freemap(128) {
    free_manager.dealloc_freemap(ptr, 128);
    println!("Deallocated 128 bytes at: {:?}", ptr);
}
```

## Error Handling
- Ensure sufficient free memory is available before attempting allocations.
- The bitmap initialization with `init_bitmap_rand()` may create unexpected results during allocation tests.
- Ensure correct deallocation to prevent fragmentation.
