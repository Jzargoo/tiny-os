struct Slab {
    free_list_head: Option<*mut ListNode>,
    next: Option<*mut Slab>,    
}

struct ListNode {
    next: Option<*mut ListNode>,
}

#[repr(C, align(16))]
struct k_mem_cache_node {
    count: usize,
    first: *mut Slab,
}

#[repr(C, align(16))]
pub struct k_mem_cache {
    object_size: usize,
    name: &'static str,
    node: *mut k_mem_cache_node,

}

#[repr(C, align(16))]
struct k_mem_cache_cpu {
    cache: *mut Slab,
    object_size: usize,
    freelist: *mut ListNode
}