#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Status"]
    pub status: crate::attiny412pac::Reg<status::STATUS_SPEC>,
    #[doc = "0x02 - Interrupt Control"]
    pub intctrl: crate::attiny412pac::Reg<intctrl::INTCTRL_SPEC>,
    #[doc = "0x03 - Interrupt Flags"]
    pub intflags: crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>,
    #[doc = "0x04 - Temporary"]
    pub temp: crate::attiny412pac::Reg<temp::TEMP_SPEC>,
    #[doc = "0x05 - Debug control"]
    pub dbgctrl: crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>,
    _reserved6: [u8; 0x01],
    #[doc = "0x07 - Clock Select"]
    pub clksel: crate::attiny412pac::Reg<clksel::CLKSEL_SPEC>,
    #[doc = "0x08 - Counter"]
    pub cnt: crate::attiny412pac::Reg<cnt::CNT_SPEC>,
    #[doc = "0x0a - Period"]
    pub per: crate::attiny412pac::Reg<per::PER_SPEC>,
    #[doc = "0x0c - Compare"]
    pub cmp: crate::attiny412pac::Reg<cmp::CMP_SPEC>,
    _reserved10: [u8; 0x02],
    #[doc = "0x10 - PIT Control A"]
    pub pitctrla: crate::attiny412pac::Reg<pitctrla::PITCTRLA_SPEC>,
    #[doc = "0x11 - PIT Status"]
    pub pitstatus: crate::attiny412pac::Reg<pitstatus::PITSTATUS_SPEC>,
    #[doc = "0x12 - PIT Interrupt Control"]
    pub pitintctrl: crate::attiny412pac::Reg<pitintctrl::PITINTCTRL_SPEC>,
    #[doc = "0x13 - PIT Interrupt Flags"]
    pub pitintflags: crate::attiny412pac::Reg<pitintflags::PITINTFLAGS_SPEC>,
    _reserved14: [u8; 0x01],
    #[doc = "0x15 - PIT Debug control"]
    pub pitdbgctrl: crate::attiny412pac::Reg<pitdbgctrl::PITDBGCTRL_SPEC>,
}
#[doc = "CLKSEL register accessor: an alias for `Reg<CLKSEL_SPEC>`"]
pub type CLKSEL = crate::attiny412pac::Reg<clksel::CLKSEL_SPEC>;
#[doc = "Clock Select"]
pub mod clksel;
#[doc = "CMP register accessor: an alias for `Reg<CMP_SPEC>`"]
pub type CMP = crate::attiny412pac::Reg<cmp::CMP_SPEC>;
#[doc = "Compare"]
pub mod cmp;
#[doc = "CNT register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::attiny412pac::Reg<cnt::CNT_SPEC>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug control"]
pub mod dbgctrl;
#[doc = "INTCTRL register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::attiny412pac::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "PER register accessor: an alias for `Reg<PER_SPEC>`"]
pub type PER = crate::attiny412pac::Reg<per::PER_SPEC>;
#[doc = "Period"]
pub mod per;
#[doc = "PITCTRLA register accessor: an alias for `Reg<PITCTRLA_SPEC>`"]
pub type PITCTRLA = crate::attiny412pac::Reg<pitctrla::PITCTRLA_SPEC>;
#[doc = "PIT Control A"]
pub mod pitctrla;
#[doc = "PITDBGCTRL register accessor: an alias for `Reg<PITDBGCTRL_SPEC>`"]
pub type PITDBGCTRL = crate::attiny412pac::Reg<pitdbgctrl::PITDBGCTRL_SPEC>;
#[doc = "PIT Debug control"]
pub mod pitdbgctrl;
#[doc = "PITINTCTRL register accessor: an alias for `Reg<PITINTCTRL_SPEC>`"]
pub type PITINTCTRL = crate::attiny412pac::Reg<pitintctrl::PITINTCTRL_SPEC>;
#[doc = "PIT Interrupt Control"]
pub mod pitintctrl;
#[doc = "PITINTFLAGS register accessor: an alias for `Reg<PITINTFLAGS_SPEC>`"]
pub type PITINTFLAGS = crate::attiny412pac::Reg<pitintflags::PITINTFLAGS_SPEC>;
#[doc = "PIT Interrupt Flags"]
pub mod pitintflags;
#[doc = "PITSTATUS register accessor: an alias for `Reg<PITSTATUS_SPEC>`"]
pub type PITSTATUS = crate::attiny412pac::Reg<pitstatus::PITSTATUS_SPEC>;
#[doc = "PIT Status"]
pub mod pitstatus;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::attiny412pac::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TEMP register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::attiny412pac::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary"]
pub mod temp;
