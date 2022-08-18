#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Status"]
    pub status: crate::attiny412pac::Reg<status::STATUS_SPEC>,
    #[doc = "0x02 - Interrupt Level 0 Priority"]
    pub lvl0pri: crate::attiny412pac::Reg<lvl0pri::LVL0PRI_SPEC>,
    #[doc = "0x03 - Interrupt Level 1 Priority Vector"]
    pub lvl1vec: crate::attiny412pac::Reg<lvl1vec::LVL1VEC_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "LVL0PRI register accessor: an alias for `Reg<LVL0PRI_SPEC>`"]
pub type LVL0PRI = crate::attiny412pac::Reg<lvl0pri::LVL0PRI_SPEC>;
#[doc = "Interrupt Level 0 Priority"]
pub mod lvl0pri;
#[doc = "LVL1VEC register accessor: an alias for `Reg<LVL1VEC_SPEC>`"]
pub type LVL1VEC = crate::attiny412pac::Reg<lvl1vec::LVL1VEC_SPEC>;
#[doc = "Interrupt Level 1 Priority Vector"]
pub mod lvl1vec;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::attiny412pac::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
