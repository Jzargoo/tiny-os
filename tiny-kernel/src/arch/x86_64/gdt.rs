use spin::Once;
use x86_64::{VirtAddr, instructions::tables::load_tss, registers::segmentation::Segment, structures::{gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector}, tss::TaskStateSegment}};


#[unsafe(link_section = ".bss")]
#[unsafe(no_mangle)]
static KERNEL_STACK: [u8; 16*1024] = [0u8; 16 * 1024];

#[unsafe(link_section = ".bss")]
#[unsafe(no_mangle)]
static KERNEL_STACK_IST: [u8; 8*1024] = [0u8; 8 * 1024];

struct Selectors {
    kc_selector: SegmentSelector,
    kd_selector: SegmentSelector,
    tss: SegmentSelector
}

struct GdtData{
    gdt: GlobalDescriptorTable,
    selectors: Selectors
}

static TSS:Once<TaskStateSegment> = Once::new(); 

static GDT:Once<GdtData> = Once::new(); 

pub fn setup_gdt() {

    let tss = TSS.call_once(|| {
        let kernel_start = VirtAddr::new(KERNEL_STACK.as_ptr() as u64);
    
        let kernel_end = kernel_start + KERNEL_STACK.len() as u64;
    
        let mut tss = TaskStateSegment::new();
    
        let ist_kernel_start = VirtAddr::new(KERNEL_STACK_IST.as_ptr() as u64);
    
        let ist_kernel_end = ist_kernel_start + KERNEL_STACK_IST.len() as u64;
        

        tss.privilege_stack_table[0] = kernel_end;

        tss.interrupt_stack_table[1] = ist_kernel_end;
    
        tss
    });

    let gdt_data = GDT.call_once(|| { 
        let mut gdt = GlobalDescriptorTable::new(); 


        let tss_s  = gdt.append(Descriptor::tss_segment(tss));


        let kcs = gdt.append(
        Descriptor::kernel_code_segment(),
        );

        let kds = gdt.append(
            Descriptor::kernel_data_segment(),
        );

        let selectors = Selectors{
            kc_selector: kcs,
            kd_selector: kds,
            tss: tss_s
        };

        GdtData { gdt, selectors }
    });

    gdt_data.gdt.load();

    unsafe {
        x86_64::instructions::segmentation::CS::set_reg(gdt_data.selectors.kc_selector);
        x86_64::instructions::segmentation::SS::set_reg(gdt_data.selectors.kd_selector);
        load_tss(gdt_data.selectors.tss);
    }

}