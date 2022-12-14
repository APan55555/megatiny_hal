#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Control B"]
    pub ctrlb: crate::attiny412pac::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x02 - Control C"]
    pub ctrlc: crate::attiny412pac::Reg<ctrlc::CTRLC_SPEC>,
    #[doc = "0x03 - Control D"]
    pub ctrld: crate::attiny412pac::Reg<ctrld::CTRLD_SPEC>,
    #[doc = "0x04 - Control E"]
    pub ctrle: crate::attiny412pac::Reg<ctrle::CTRLE_SPEC>,
    _reserved5: [u8; 0x03],
    #[doc = "0x08 - EVCTRLA"]
    pub evctrla: crate::attiny412pac::Reg<evctrla::EVCTRLA_SPEC>,
    #[doc = "0x09 - EVCTRLB"]
    pub evctrlb: crate::attiny412pac::Reg<evctrlb::EVCTRLB_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x0c - Interrupt Control"]
    pub intctrl: crate::attiny412pac::Reg<intctrl::INTCTRL_SPEC>,
    #[doc = "0x0d - Interrupt Flags"]
    pub intflags: crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>,
    #[doc = "0x0e - Status"]
    pub status: crate::attiny412pac::Reg<status::STATUS_SPEC>,
    _reserved10: [u8; 0x01],
    #[doc = "0x10 - Input Control A"]
    pub inputctrla: crate::attiny412pac::Reg<inputctrla::INPUTCTRLA_SPEC>,
    #[doc = "0x11 - Input Control B"]
    pub inputctrlb: crate::attiny412pac::Reg<inputctrlb::INPUTCTRLB_SPEC>,
    #[doc = "0x12 - Fault Control"]
    pub faultctrl: crate::attiny412pac::Reg<faultctrl::FAULTCTRL_SPEC>,
    _reserved13: [u8; 0x01],
    #[doc = "0x14 - Delay Control"]
    pub dlyctrl: crate::attiny412pac::Reg<dlyctrl::DLYCTRL_SPEC>,
    #[doc = "0x15 - Delay value"]
    pub dlyval: crate::attiny412pac::Reg<dlyval::DLYVAL_SPEC>,
    _reserved15: [u8; 0x02],
    #[doc = "0x18 - Dither Control A"]
    pub ditctrl: crate::attiny412pac::Reg<ditctrl::DITCTRL_SPEC>,
    #[doc = "0x19 - Dither value"]
    pub ditval: crate::attiny412pac::Reg<ditval::DITVAL_SPEC>,
    _reserved17: [u8; 0x04],
    #[doc = "0x1e - Debug Control"]
    pub dbgctrl: crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>,
    _reserved18: [u8; 0x03],
    #[doc = "0x22 - Capture A"]
    pub capturea: crate::attiny412pac::Reg<capturea::CAPTUREA_SPEC>,
    #[doc = "0x24 - Capture B"]
    pub captureb: crate::attiny412pac::Reg<captureb::CAPTUREB_SPEC>,
    _reserved20: [u8; 0x02],
    #[doc = "0x28 - Compare A Set"]
    pub cmpaset: crate::attiny412pac::Reg<cmpaset::CMPASET_SPEC>,
    #[doc = "0x2a - Compare A Clear"]
    pub cmpaclr: crate::attiny412pac::Reg<cmpaclr::CMPACLR_SPEC>,
    #[doc = "0x2c - Compare B Set"]
    pub cmpbset: crate::attiny412pac::Reg<cmpbset::CMPBSET_SPEC>,
    #[doc = "0x2e - Compare B Clear"]
    pub cmpbclr: crate::attiny412pac::Reg<cmpbclr::CMPBCLR_SPEC>,
}
#[doc = "CAPTUREA register accessor: an alias for `Reg<CAPTUREA_SPEC>`"]
pub type CAPTUREA = crate::attiny412pac::Reg<capturea::CAPTUREA_SPEC>;
#[doc = "Capture A"]
pub mod capturea;
#[doc = "CAPTUREB register accessor: an alias for `Reg<CAPTUREB_SPEC>`"]
pub type CAPTUREB = crate::attiny412pac::Reg<captureb::CAPTUREB_SPEC>;
#[doc = "Capture B"]
pub mod captureb;
#[doc = "CMPACLR register accessor: an alias for `Reg<CMPACLR_SPEC>`"]
pub type CMPACLR = crate::attiny412pac::Reg<cmpaclr::CMPACLR_SPEC>;
#[doc = "Compare A Clear"]
pub mod cmpaclr;
#[doc = "CMPASET register accessor: an alias for `Reg<CMPASET_SPEC>`"]
pub type CMPASET = crate::attiny412pac::Reg<cmpaset::CMPASET_SPEC>;
#[doc = "Compare A Set"]
pub mod cmpaset;
#[doc = "CMPBCLR register accessor: an alias for `Reg<CMPBCLR_SPEC>`"]
pub type CMPBCLR = crate::attiny412pac::Reg<cmpbclr::CMPBCLR_SPEC>;
#[doc = "Compare B Clear"]
pub mod cmpbclr;
#[doc = "CMPBSET register accessor: an alias for `Reg<CMPBSET_SPEC>`"]
pub type CMPBSET = crate::attiny412pac::Reg<cmpbset::CMPBSET_SPEC>;
#[doc = "Compare B Set"]
pub mod cmpbset;
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::attiny412pac::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::attiny412pac::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::attiny412pac::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "CTRLE register accessor: an alias for `Reg<CTRLE_SPEC>`"]
pub type CTRLE = crate::attiny412pac::Reg<ctrle::CTRLE_SPEC>;
#[doc = "Control E"]
pub mod ctrle;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "DITCTRL register accessor: an alias for `Reg<DITCTRL_SPEC>`"]
pub type DITCTRL = crate::attiny412pac::Reg<ditctrl::DITCTRL_SPEC>;
#[doc = "Dither Control A"]
pub mod ditctrl;
#[doc = "DITVAL register accessor: an alias for `Reg<DITVAL_SPEC>`"]
pub type DITVAL = crate::attiny412pac::Reg<ditval::DITVAL_SPEC>;
#[doc = "Dither value"]
pub mod ditval;
#[doc = "DLYCTRL register accessor: an alias for `Reg<DLYCTRL_SPEC>`"]
pub type DLYCTRL = crate::attiny412pac::Reg<dlyctrl::DLYCTRL_SPEC>;
#[doc = "Delay Control"]
pub mod dlyctrl;
#[doc = "DLYVAL register accessor: an alias for `Reg<DLYVAL_SPEC>`"]
pub type DLYVAL = crate::attiny412pac::Reg<dlyval::DLYVAL_SPEC>;
#[doc = "Delay value"]
pub mod dlyval;
#[doc = "EVCTRLA register accessor: an alias for `Reg<EVCTRLA_SPEC>`"]
pub type EVCTRLA = crate::attiny412pac::Reg<evctrla::EVCTRLA_SPEC>;
#[doc = "EVCTRLA"]
pub mod evctrla;
#[doc = "EVCTRLB register accessor: an alias for `Reg<EVCTRLB_SPEC>`"]
pub type EVCTRLB = crate::attiny412pac::Reg<evctrlb::EVCTRLB_SPEC>;
#[doc = "EVCTRLB"]
pub mod evctrlb;
#[doc = "FAULTCTRL register accessor: an alias for `Reg<FAULTCTRL_SPEC>`"]
pub type FAULTCTRL = crate::attiny412pac::Reg<faultctrl::FAULTCTRL_SPEC>;
#[doc = "Fault Control"]
pub mod faultctrl;
#[doc = "INPUTCTRLA register accessor: an alias for `Reg<INPUTCTRLA_SPEC>`"]
pub type INPUTCTRLA = crate::attiny412pac::Reg<inputctrla::INPUTCTRLA_SPEC>;
#[doc = "Input Control A"]
pub mod inputctrla;
#[doc = "INPUTCTRLB register accessor: an alias for `Reg<INPUTCTRLB_SPEC>`"]
pub type INPUTCTRLB = crate::attiny412pac::Reg<inputctrlb::INPUTCTRLB_SPEC>;
#[doc = "Input Control B"]
pub mod inputctrlb;
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
