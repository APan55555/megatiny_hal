#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Control Register B"]
    pub ctrlb: crate::attiny412pac::Reg<ctrlb::CTRLB_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Event Control"]
    pub evctrl: crate::attiny412pac::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x05 - Interrupt Control"]
    pub intctrl: crate::attiny412pac::Reg<intctrl::INTCTRL_SPEC>,
    #[doc = "0x06 - Interrupt Flags"]
    pub intflags: crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>,
    #[doc = "0x07 - Status"]
    pub status: crate::attiny412pac::Reg<status::STATUS_SPEC>,
    #[doc = "0x08 - Debug Control"]
    pub dbgctrl: crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x09 - Temporary Value"]
    pub temp: crate::attiny412pac::Reg<temp::TEMP_SPEC>,
    #[doc = "0x0a - Count"]
    pub cnt: crate::attiny412pac::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0c - Compare or Capture"]
    pub ccmp: crate::attiny412pac::Reg<ccmp::CCMP_SPEC>,
}
#[doc = "CCMP register accessor: an alias for `Reg<CCMP_SPEC>`"]
pub type CCMP = crate::attiny412pac::Reg<ccmp::CCMP_SPEC>;
#[doc = "Compare or Capture"]
pub mod ccmp;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::attiny412pac::Reg<cnt::CNT_SPEC>;
#[doc = "Count"]
pub mod cnt;
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::attiny412pac::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control Register B"]
pub mod ctrlb;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::attiny412pac::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTCTRL register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::attiny412pac::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::attiny412pac::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TEMP register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::attiny412pac::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary Value"]
pub mod temp;
