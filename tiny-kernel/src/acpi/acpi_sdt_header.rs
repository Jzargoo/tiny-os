
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
pub struct AcpiSdtHeader{
    pub signature: [u8; 4],
    pub len: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oemid: [char; 8],
    pub oem_table_id: [char; 6],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32
}