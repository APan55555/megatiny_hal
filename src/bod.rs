#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x06],
    #[doc = "0x08 - Voltage level monitor Control"]
    pub vlmctrla: crate::Reg<vlmctrla::VLMCTRLA_SPEC>,
    #[doc = "0x09 - Voltage level monitor interrupt Control"]
    pub intctrl: crate::Reg<intctrl::INTCTRL_SPEC>,
    #[doc = "0x0a - Voltage level monitor interrupt Flags"]
    pub intflags: crate::Reg<intflags::INTFLAGS_SPEC>,
    #[doc = "0x0b - Voltage level monitor status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "INTCTRL register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Voltage level monitor interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Voltage level monitor interrupt Flags"]
pub mod intflags;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Voltage level monitor status"]
pub mod status;
#[doc = "VLMCTRLA register accessor: an alias for `Reg<VLMCTRLA_SPEC>`"]
pub type VLMCTRLA = crate::Reg<vlmctrla::VLMCTRLA_SPEC>;
#[doc = "Voltage level monitor Control"]
pub mod vlmctrla;
