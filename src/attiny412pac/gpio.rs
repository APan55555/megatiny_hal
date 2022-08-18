#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General Purpose IO Register 0"]
    pub gpior0: crate::attiny412pac::Reg<gpior0::GPIOR0_SPEC>,
    #[doc = "0x01 - General Purpose IO Register 1"]
    pub gpior1: crate::attiny412pac::Reg<gpior1::GPIOR1_SPEC>,
    #[doc = "0x02 - General Purpose IO Register 2"]
    pub gpior2: crate::attiny412pac::Reg<gpior2::GPIOR2_SPEC>,
    #[doc = "0x03 - General Purpose IO Register 3"]
    pub gpior3: crate::attiny412pac::Reg<gpior3::GPIOR3_SPEC>,
}
#[doc = "GPIOR0 register accessor: an alias for `Reg<GPIOR0_SPEC>`"]
pub type GPIOR0 = crate::attiny412pac::Reg<gpior0::GPIOR0_SPEC>;
#[doc = "General Purpose IO Register 0"]
pub mod gpior0;
#[doc = "GPIOR1 register accessor: an alias for `Reg<GPIOR1_SPEC>`"]
pub type GPIOR1 = crate::attiny412pac::Reg<gpior1::GPIOR1_SPEC>;
#[doc = "General Purpose IO Register 1"]
pub mod gpior1;
#[doc = "GPIOR2 register accessor: an alias for `Reg<GPIOR2_SPEC>`"]
pub type GPIOR2 = crate::attiny412pac::Reg<gpior2::GPIOR2_SPEC>;
#[doc = "General Purpose IO Register 2"]
pub mod gpior2;
#[doc = "GPIOR3 register accessor: an alias for `Reg<GPIOR3_SPEC>`"]
pub type GPIOR3 = crate::attiny412pac::Reg<gpior3::GPIOR3_SPEC>;
#[doc = "General Purpose IO Register 3"]
pub mod gpior3;
