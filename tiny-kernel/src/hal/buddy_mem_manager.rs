use core::ptr::slice_from_raw_parts_mut;
use x86_64::structures::paging::RecursivePageTable;

use crate::hal::kernel_allocator::BumpAllocator;
use crate::hal::page_allocator::KernelMemRegion;
use crate::println; 

const MIN_ORDER: u8 = 12; // 4KB 
const MAX_ORDER: u8 = 21; // 2MB 

#[allow(dead_code)]
struct BuddyRoot {
    next: Option<*mut BuddyRoot>,
    start: u64,
    bitmap: *mut [bool],
    order: u8, 
}

pub struct DeallocationError<'a> {
    pub error_message: &'a str
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BuddyManager {
    buddy_root: Option<*mut BuddyRoot>,
    k_offset: u64
}

impl BuddyRoot {
    fn find_free_block(&mut self, generation: u8) -> Option<usize> { 
        let bitmap = unsafe {
            self.bitmap.as_mut().unwrap_or_else(|| {
                println!("[BUDDY] CRITICAL: Bitmap in buddy root was not initialized!");
                panic!();
            })
        };

        let idx = BuddyRoot::find_free_index_by_bitmap(generation, bitmap, 0)?;
        let block_size = 1 << (self.order - generation);
        let local_idx = idx - ((1 << generation) - 1);
        let offset = local_idx * block_size;

        Some(offset)
    }

    fn find_free_index_by_bitmap(target_generation: u8, bitmap: &mut[bool], sub_tree: usize) -> Option<usize> { 
        if sub_tree >= bitmap.len() {
            return None;
        }

        let curr_gen = ((sub_tree + 1).ilog2()) as u8;

        if curr_gen == target_generation {
            if bitmap[sub_tree] {
                return None;
            } else {
                bitmap[sub_tree] = true;
                return Some(sub_tree);
            }
        } 
        
        let left_child_idx = (sub_tree * 2) + 1; 
        if let Some(idx) = BuddyRoot::find_free_index_by_bitmap(target_generation, bitmap, left_child_idx) {
            bitmap[sub_tree] = true;
            return Some(idx);
        }
        
        let right_child_idx = (sub_tree * 2) + 2;
        if let Some(idx) = BuddyRoot::find_free_index_by_bitmap(target_generation, bitmap, right_child_idx) {
            bitmap[sub_tree] = true;
            return Some(idx);
        }

        None
    }
}

// GENERATION INDEX is the index of current generation without any affects of the other layers.
// For example 
//   0
// 1    2 their indexes. GENERATION INDEX looks like index of [1,2] 
impl BuddyRoot {
    fn free<'a>(&mut self, generation: u8, frame_start: u64) -> Result<(), DeallocationError<'a> >{
        
        let frame_offset = frame_start - self.start;
        
        let node_size = 1 << (self.order - generation); 

        let bitmap = unsafe {
            self.bitmap.as_mut().unwrap_or_else(
                || {
                    println!("Error. Buddy root is incorrect and did not have bitmap.");
                    panic!()
                }
            )   
        };
        
        if frame_offset % node_size != 0 {
            
            #[cfg(debug_assertions)]
            println!("[BUDDY] error: Frame start address was not aligned: address {} and subsequent offset: {} not aligned with offset {} and root start: {}", frame_start, frame_offset,node_size, self.start);
            
            return Err(
                DeallocationError { 
                    error_message: "Frame start address was not aligned on nodes in the provided generation" 
                }
            );

        } 

        let gen_index = frame_offset as usize / 1 << (self.order - generation);

        BuddyRoot::recursive_freeing_nodes(bitmap, gen_index, generation);
    
        Ok(())
        
    } 

    fn recursive_freeing_nodes(bitmap: &mut[bool], generation_index: usize, generation: u8) {

        let bitmap_index = BuddyRoot::get_index_by_generation_index(generation_index, generation);
        
        if generation == 0 {

            #[cfg(debug_assertions)]
            println!("[BUDDY] a root was freed!");

            bitmap[bitmap_index] = false;
            return;
        }

        let buddy = if generation_index % 2 == 0 {
            bitmap_index + 1
        } else {
            bitmap_index - 1
        };


        bitmap[bitmap_index] = false;

        if bitmap[buddy] {
            #[cfg(debug_assertions)]
            println!("[BUDDY] buddy with index {} is busy", buddy);

            return;
        } else {
            let parent_index = generation_index / 2;
            BuddyRoot::recursive_freeing_nodes(bitmap, parent_index, generation -1);
        }
    }

    fn get_index_by_generation_index(gener_index:usize, generation: u8) -> usize{
        gener_index * generation as usize    
    }
}

#[allow(dead_code)]
impl BuddyManager {
    
    pub fn new(physical_offset: u64) -> Self {
        Self { 
            buddy_root: None,
            k_offset: physical_offset
        }
    }

    pub fn add_region(&mut self, start_addr: *mut u8, mut size: usize, kernel_alloc: &mut BumpAllocator) {
        let mut curr_addr = start_addr;

        println!("[BUDDY] Adding memory region: base={:p}, size={} bytes", start_addr, size);

        while size != 0 {
            let mut order = size.ilog2() as u8;
            
            if order < MIN_ORDER {
                println!("[BUDDY] Trailing block size {} bytes is less than MIN_ORDER (4KB). Ignoring.", size);
                return;
            } else if order > MAX_ORDER {
                order = MAX_ORDER;
            }

            let root_ptr = match kernel_alloc.k_alloc(size_of::<BuddyRoot>(), align_of::<BuddyRoot>()) {
                Some(ptr) => ptr as *mut BuddyRoot,
                None => {
                    println!("[BUDDY] CRITICAL: Out of kernel memory while allocating BuddyRoot structure!");
                    panic!();
                }
            };

            let leaves_count = 1 << (order - MIN_ORDER);
            let bitmap_len = (leaves_count * 2) - 1;

            let bitmap_ptr = match kernel_alloc.k_alloc(bitmap_len, align_of::<bool>()) {
                Some(ptr) => ptr as *mut bool,
                None => {
                    println!("[BUDDY] CRITICAL: Out of kernel memory while allocating allocator bitmap!");
                    panic!();
                }
            };

            unsafe {
                core::ptr::write_bytes(bitmap_ptr, 0u8, bitmap_len);
                let bitmap: *mut [bool] = slice_from_raw_parts_mut(bitmap_ptr, bitmap_len);

                root_ptr.write(BuddyRoot { 
                    next: self.buddy_root,
                    start: curr_addr as u64,
                    bitmap,
                    order
                });
            }

            self.buddy_root = Some(root_ptr);
            let block_size = 1usize << (order as usize);

            println!("[BUDDY]   Created Root Node: base=0x{:x}, order={}, elements={}", curr_addr as usize, order, bitmap_len);

            curr_addr = unsafe { curr_addr.add(block_size) };
            size -= block_size;
        }
    }

    
    pub fn  deallocate_bytes<'a>(&mut self, frame_start: u64, frame_size_bytes: usize) -> Result<(), DeallocationError<'a> >{
        let order = calculate_order_ceil(frame_size_bytes);

        let mut current = self.buddy_root;

        while let Some(node_ptr) = current {
            
                let root = unsafe {
                    node_ptr.as_mut().unwrap_or_else(|| {
                        println!("[BUDDY] CRITICAL: Corrupted linked list! Invalid buddy root pointer encountered.");
                        panic!();
                    })
                };


            if root.start <= frame_start && frame_start <= (root.start + (1 << root.order))  {
                root.free(root.order  - order, frame_start)?;
            } else {
                current = root.next
            }
        }

        Ok(())
    }

    pub fn allocate_bytes(&mut self, frame_size_bytes: usize) -> Option<KernelMemRegion> {
        let order = calculate_order_ceil(frame_size_bytes);

        if order < MIN_ORDER {
            println!("[BUDDY] Allocation failed: requested {} bytes (less than 4KB page boundary)", frame_size_bytes);
            return None;
        } else if order > MAX_ORDER {
            println!("[BUDDY] Allocation failed: requested {} bytes exceeds MAX_ORDER ({})", frame_size_bytes, MAX_ORDER);
            return None;
        }

        let mut current = self.buddy_root;

        while let Some(node_ptr) = current {
            unsafe {
                let root = node_ptr.as_mut().unwrap_or_else(|| {
                    println!("[BUDDY] CRITICAL: Corrupted linked list! Invalid buddy root pointer encountered.");
                    panic!();
                });
                
                if order <= root.order {
                    if let Some(block) = root.find_free_block(root.order - order) {
                        let v_start = root.start + block as u64;
                        
                        println!("[BUDDY] Allocated block: phys=0x{:x}, requested_bytes={}", v_start, frame_size_bytes);

                        let region = KernelMemRegion {
                            start_paddr: v_start,  
                            length_bytes: 1 << order
                        };

                        return Some(region);
                    }
                }
                current = root.next;
            }
        }

        println!("[BUDDY] OOM: No free block of order {} found for requested {} bytes!", order, frame_size_bytes);
        None
    }
}

fn calculate_order_ceil(size: usize) -> u8 {
    if size <= 1 {
        0
    } else {
        ((size - 1).ilog2() + 1) as u8
    }
}