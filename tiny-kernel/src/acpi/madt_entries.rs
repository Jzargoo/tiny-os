
#[allow(unused)]
#[repr(C, packed)]
pub struct ProcessorLapic{
    pub processor_id: u8,
    pub apic_id: u8,
    pub flags: u32
}


#[allow(unused)]
#[repr(C, packed)]
pub struct IOApic{
    pub io_apic_id: u8,
    __: u8,
    pub io_apic_address: u32,
    pub global_interrupt: u32
}

#[allow(unused)]
#[repr(C, packed)]
pub struct IOApicSourceOverride{
    pub bus_source: u8,
    pub irq_source: u8,
    pub gsi: u32,
    pub flags: u16
}


#[allow(unused)]
#[repr(C, packed)]
pub struct IOApicNonMaskableInterruptSource{
    pub nmi_source: u8,
    __: u8,
    pub flags: u16,
    pub gsi: u32
}


#[allow(unused)]
#[repr(C, packed)]
pub struct LapicNonMaskableInterrupts{
    pub acpi_processor_id: u8,
    pub flags: u16,
    pub lint: u8
}


#[allow(unused)]
#[repr(C, packed)]
pub struct LapicAddressOverride{
    __: u16,
    pub lapic_address: u64 
}

#[repr(C, packed)]
#[allow(unused)]
pub struct X2LapicProcessor{
    __: u16,
    pub processor_x2lapic_id: u32,
    pub flags: u32,
    pub acpi_id: u32
}

#[allow(unused)]
pub enum EntryType {
    EntryType0(&'static ProcessorLapic),
    EntryType1(&'static IOApic),
    EntryType2(&'static IOApicSourceOverride),
    EntryType3(&'static IOApicNonMaskableInterruptSource),
    EntryType4(&'static LapicNonMaskableInterrupts),
    EntryType5(&'static LapicAddressOverride),
    EntryType9(&'static X2LapicProcessor),
    EntryTypeUnknown(u8)
}

impl EntryType {

    pub fn parse(entry_type: u8, payload: &[u8]) -> Self{
        
        unsafe { 
        
            match entry_type {

                0 if payload.len() >= size_of::<ProcessorLapic>() => 
                    EntryType::EntryType0(
                    
                        & ( *(payload.as_ptr() as *const ProcessorLapic) )
                    
                    ),

            
                1 if payload.len() >= size_of::<IOApic>() => 
                    EntryType::EntryType1(
                    
                        & ( *(payload.as_ptr() as *const IOApic) )
                    
                    ),

                2 if payload.len() >= size_of::<IOApicSourceOverride>() => 
                    EntryType::EntryType2(
                    
                        & ( *(payload.as_ptr() as *const IOApicSourceOverride) )
                    
                    ),

                3 if payload.len() >= size_of::<IOApicNonMaskableInterruptSource>() => 
                    EntryType::EntryType3(
                    
                        & ( *(payload.as_ptr() as *const IOApicNonMaskableInterruptSource) )
                    
                    ),
                
                4 if payload.len() >= size_of::<LapicNonMaskableInterrupts>() => 
                    EntryType::EntryType4(
                    
                        & ( *(payload.as_ptr() as *const LapicNonMaskableInterrupts) )
                    
                    ),
                
                5 if payload.len() >= size_of::<LapicAddressOverride>() => 
                    EntryType::EntryType5(
                    
                        & ( *(payload.as_ptr() as *const LapicAddressOverride) )
                    
                    ),

                9 if payload.len() >= size_of::<X2LapicProcessor>() => 
                    EntryType::EntryType9(
                    
                        & ( *(payload.as_ptr() as *const X2LapicProcessor) )
                    
                    ),

                _ => EntryType::EntryTypeUnknown(entry_type) 
        
            }
        
        }

    }
}