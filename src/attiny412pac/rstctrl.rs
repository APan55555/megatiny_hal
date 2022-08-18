#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Flags"]
    pub rstfr: crate::attiny412pac::Reg<rstfr::RSTFR_SPEC>,
    #[doc = "0x01 - Software Reset"]
    pub swrr: crate::attiny412pac::Reg<swrr::SWRR_SPEC>,
}
#[doc = "RSTFR register accessor: an alias for `Reg<RSTFR_SPEC>`"]
pub type RSTFR = crate::attiny412pac::Reg<rstfr::RSTFR_SPEC>;
#[doc = "Reset Flags"]
pub mod rstfr;
#[doc = "SWRR register accessor: an alias for `Reg<SWRR_SPEC>`"]
pub type SWRR = crate::attiny412pac::Reg<swrr::SWRR_SPEC>;
#[doc = "Software Reset"]
pub mod swrr;
