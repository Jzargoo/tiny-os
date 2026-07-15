
#[repr(C, packed)]
pub struct AcpiSdtHeader{
    signature: [char; 4],
    len: u32,
    revision: u8,
    checksum: u8,
    oemid: [char; 8],
    oem_table_id: [char; 6],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32
}