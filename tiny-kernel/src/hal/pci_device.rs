use core::fmt::Debug;

#[allow(dead_code)]
pub struct PciDevice {
    vendor_id: u16,
    device_id: u16,
    status: u16,
    command: u16,
    class: u8,
    sub_class: u8,
    bus: u8,
    slot: u8, 
    function: u8,
    revision: u8,
    prog_if: u8,
    bist: u8,
    header_type: u8,
    timer: u8,
    cache_line_size: u8,
}

impl PciDevice {
    pub fn new (
        vendor_id: u16,
        device_id: u16,
        status: u16,
        command: u16,
        class: u8,
        sub_class: u8,
        bus: u8,
        slot: u8, 
        function: u8,
        revision: u8,
        prog_if: u8,
        bist: u8,
        header_type: u8,
        timer: u8,
        cache_line_size: u8
    ) -> Self {
        Self { 
            vendor_id,
            device_id, 
            status, 
            command, 
            class, sub_class,
            bus, slot, function,
            revision, 
            prog_if, 
            bist, 
            header_type, 
            timer, 
            cache_line_size 
        }
    }
}

impl Debug for PciDevice {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PciDevice").field("vendor_id", &self.vendor_id).field("device_id", &self.device_id).finish()
    }
}