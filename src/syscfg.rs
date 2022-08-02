#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    #[doc = "0x01 - Revision ID"]
    pub revid: crate::Reg<revid::REVID_SPEC>,
    #[doc = "0x02 - External Break"]
    pub extbrk: crate::Reg<extbrk::EXTBRK_SPEC>,
}
#[doc = "EXTBRK register accessor: an alias for `Reg<EXTBRK_SPEC>`"]
pub type EXTBRK = crate::Reg<extbrk::EXTBRK_SPEC>;
#[doc = "External Break"]
pub mod extbrk;
#[doc = "REVID register accessor: an alias for `Reg<REVID_SPEC>`"]
pub type REVID = crate::Reg<revid::REVID_SPEC>;
#[doc = "Revision ID"]
pub mod revid;
