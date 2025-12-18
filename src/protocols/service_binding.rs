//! Service Binding Protocol
//!
//! Provides services that are required to create and destroy child handles
//! that support a given set of protocols.

pub type ProtocolCreateChild = eficall! {unsafe fn(
    *mut Protocol,
    *mut crate::base::Handle,
) -> crate::base::Status};

pub type ProtocolDestroyChild = eficall! {unsafe fn(
    *mut Protocol,
    crate::base::Handle,
) -> crate::base::Status};

#[repr(C)]
pub struct Protocol {
    pub create_child: ProtocolCreateChild,
    pub destroy_child: ProtocolDestroyChild,
}
