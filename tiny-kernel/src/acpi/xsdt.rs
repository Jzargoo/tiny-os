use crate::acpi::acpi_sdt_header::AcpiSdtHeader;

#[repr(C, packed)]
pub struct Xsdt{
    acpi_sdt_header: AcpiSdtHeader,
    other_tables_addres: u64
}