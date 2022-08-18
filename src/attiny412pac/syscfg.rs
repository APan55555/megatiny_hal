#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Revision ID"]
    pub revid: crate::attiny412pac::Reg<revid::REVID_SPEC>,
    #[doc = "0x01 - External Break"]
    pub extbrk: crate::attiny412pac::Reg<extbrk::EXTBRK_SPEC>,
}
#[doc = "EXTBRK register accessor: an alias for `Reg<EXTBRK_SPEC>`"]
pub type EXTBRK = crate::attiny412pac::Reg<extbrk::EXTBRK_SPEC>;
#[doc = "External Break"]
pub mod extbrk;
#[doc = "REVID register accessor: an alias for `Reg<REVID_SPEC>`"]
pub type REVID = crate::attiny412pac::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
