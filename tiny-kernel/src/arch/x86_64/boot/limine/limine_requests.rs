use limine::{
    self, BaseRevision, RequestsEndMarker, RequestsStartMarker, request::{
        EntryPointRequest,  FramebufferRequest, HhdmRequest, MemmapRequest, RsdpRequest, StackSizeRequest
    }
};

use crate::arch::x86_64::boot::limine::starter::_start;

#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".requests_start")]
pub static REQUESTS_START: RequestsStartMarker = RequestsStartMarker::new();



#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section = ".requests")]
pub static REQUESTS_RSDP: RsdpRequest = RsdpRequest::new();




#[unsafe(no_mangle)]
#[used]
#[unsafe(link_section=".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();



#[used]
#[unsafe(link_section = ".requests")]
pub static STACK: StackSizeRequest = StackSizeRequest::new(1024 * 1024 * 16);



#[unsafe(link_section = ".requests")]
pub static HHDM: HhdmRequest = HhdmRequest::new();



#[unsafe(link_section = ".requests")]
pub static MEMMAP: MemmapRequest = MemmapRequest::new();



#[used]
#[unsafe(link_section=".requests")]
pub static ENTRY_REQUEST: EntryPointRequest = EntryPointRequest::new(_start);



#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();



#[used]
#[unsafe(link_section = ".requests_end")]
pub static REQUESTS_END: RequestsEndMarker = RequestsEndMarker::new();
