use crate::{acpi::{facp::Facp, hpet::Hpet, madt::Madt, mcfg::Mcfg, xsdt::Xsdt}, hal::addresses::PhysicalAddress};

pub struct  TableRegistry <P: PhysicalAddress> {
    pub xsdt: Xsdt<P>,
    pub facp: Option<Facp<P>>,
    pub madt: Option<Madt<P>>,
    pub hpet: Option<Hpet<P>>,
    pub mcfg: Option<Mcfg<P>>,
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
        facp: Option<Facp<P>>,
        madt: Option<Madt<P>>,
        hpet: Option<Hpet<P>>,
        mcfg: Option<Mcfg<P>>,
    ) -> Self { 
        TableRegistry{
            xsdt, facp, madt, mcfg, hpet
        }
    }

}