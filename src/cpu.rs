#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - Configuration Change Protection"]
    pub ccp: crate::Reg<ccp::CCP_SPEC>,
    _reserved1: [u8; 0x08],
    #[doc = "0x0d - Stack Pointer Low"]
    pub spl: crate::Reg<spl::SPL_SPEC>,
    #[doc = "0x0e - Stack Pointer High"]
    pub sph: crate::Reg<sph::SPH_SPEC>,
}
#[doc = "CCP register accessor: an alias for `Reg<CCP_SPEC>`"]
pub type CCP = crate::Reg<ccp::CCP_SPEC>;
#[doc = "Configuration Change Protection"]
pub mod ccp;
#[doc = "SPH register accessor: an alias for `Reg<SPH_SPEC>`"]
pub type SPH = crate::Reg<sph::SPH_SPEC>;
#[doc = "Stack Pointer High"]
pub mod sph;
#[doc = "SPL register accessor: an alias for `Reg<SPL_SPEC>`"]
pub type SPL = crate::Reg<spl::SPL_SPEC>;
#[doc = "Stack Pointer Low"]
pub mod spl;
