use crate::{acpi::{facp::Facp, hpet::Hpet, madt::Madt, mcfg::Mcfg, xsdt::Xsdt}, hal::addresses::PhysicalAddress};

pub struct  TableRegistry <P: PhysicalAddress> {
    xsdt: Xsdt<P>,
    facp: Facp<P>,
    madt: Madt<P>,
    hpet: Hpet<P>,
    mcfg: Mcfg<P>,
}

pub enum Tables {
    XSDT,
    FACP,
    MCFG,
    HPET,
    MADT
}

impl Tables {
    pub fn get_signature(&self) -> [u8; 4] {
        match self {
            Tables::XSDT => *b"XSDT",
            Tables::FACP => *b"FACP",
            Tables::MCFG => *b"MCFG",
            Tables::HPET => *b"HPET",
            Tables::MADT => *b"ACPI"
        }
    }
}

impl <P: PhysicalAddress> TableRegistry<P> {
    
    pub fn new(
        xsdt: Xsdt<P>,
        facp: Facp<P>,
        madt: Madt<P>,
        hpet: Hpet<P>,
        mcfg: Mcfg<P>,
    ) -> Self { 
        TableRegistry{
            xsdt, facp, madt, mcfg, hpet
        }
    }

}