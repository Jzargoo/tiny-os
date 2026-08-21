use core::{cmp::Ordering, sync::atomic::AtomicUsize};

use lapic::EndOfInterrupt;
use spin::Mutex;

static LAPIC_ADDR: AtomicUsize = AtomicUsize::new(0);

pub(super) unsafe fn lapic_send_eoi(){

    if LAPIC_ADDR.load(core::sync::atomic::Ordering::Relaxed) == 0 {
        panic!("Lapic address is not initialized yet, however there was a call to send EOI to LAPIC(addr is not known)");
    }

    let mut eoi = EndOfInterrupt::new();

    eoi.set_eoi(0);

    let bytes = eoi.into_bytes();


} 