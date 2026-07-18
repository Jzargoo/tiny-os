

#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AcpiSdtHeader{
    pub signature: [u8; 4],
    pub len: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32
}

impl AcpiSdtHeader {
    pub fn get_raw_data_addres(&self) -> usize {
        self as *const _ as usize + size_of::<AcpiSdtHeader>()
    }

    pub fn get_data_len(&self) -> usize {
        let total_size = self.len as usize;
        let header_size = size_of::<AcpiSdtHeader>();
        total_size.saturating_sub(header_size)
    }
}