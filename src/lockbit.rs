#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Lock bits"]
    pub lockbit: crate::Reg<lockbit::LOCKBIT_SPEC>,
}
#[doc = "LOCKBIT register accessor: an alias for `Reg<LOCKBIT_SPEC>`"]
pub type LOCKBIT = crate::Reg<lockbit::LOCKBIT_SPEC>;
#[doc = "Lock bits"]
pub mod lockbit;
