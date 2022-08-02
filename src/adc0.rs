#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x02 - Control C"]
    pub ctrlc: crate::Reg<ctrlc::CTRLC_SPEC>,
    #[doc = "0x03 - Control D"]
    pub ctrld: crate::Reg<ctrld::CTRLD_SPEC>,
    #[doc = "0x04 - Control E"]
    pub ctrle: crate::Reg<ctrle::CTRLE_SPEC>,
    #[doc = "0x05 - Sample Control"]
    pub sampctrl: crate::Reg<sampctrl::SAMPCTRL_SPEC>,
    #[doc = "0x06 - Positive mux input"]
    pub muxpos: crate::Reg<muxpos::MUXPOS_SPEC>,
    _reserved7: [u8; 0x01],
    #[doc = "0x08 - Command"]
    pub command: crate::Reg<command::COMMAND_SPEC>,
    #[doc = "0x09 - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x0a - Interrupt Control"]
    pub intctrl: crate::Reg<intctrl::INTCTRL_SPEC>,
    #[doc = "0x0b - Interrupt Flags"]
    pub intflags: crate::Reg<intflags::INTFLAGS_SPEC>,
    #[doc = "0x0c - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x0d - Temporary Data"]
    pub temp: crate::Reg<temp::TEMP_SPEC>,
    _reserved13: [u8; 0x02],
    #[doc = "0x10 - ADC Accumulator Result"]
    pub res: crate::Reg<res::RES_SPEC>,
    #[doc = "0x12 - Window comparator low threshold"]
    pub winlt: crate::Reg<winlt::WINLT_SPEC>,
    #[doc = "0x14 - Window comparator high threshold"]
    pub winht: crate::Reg<winht::WINHT_SPEC>,
    #[doc = "0x16 - Calibration"]
    pub calib: crate::Reg<calib::CALIB_SPEC>,
}
#[doc = "CALIB register accessor: an alias for `Reg<CALIB_SPEC>`"]
pub type CALIB = crate::Reg<calib::CALIB_SPEC>;
#[doc = "Calibration"]
pub mod calib;
#[doc = "COMMAND register accessor: an alias for `Reg<COMMAND_SPEC>`"]
pub type COMMAND = crate::Reg<command::COMMAND_SPEC>;
#[doc = "Command"]
pub mod command;
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Control B"]
pub mod ctrlb;
#[doc = "CTRLC register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Control C"]
pub mod ctrlc;
#[doc = "CTRLD register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Control D"]
pub mod ctrld;
#[doc = "CTRLE register accessor: an alias for `Reg<CTRLE_SPEC>`"]
pub type CTRLE = crate::Reg<ctrle::CTRLE_SPEC>;
#[doc = "Control E"]
pub mod ctrle;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "INTCTRL register accessor: an alias for `Reg<INTCTRL_SPEC>`"]
pub type INTCTRL = crate::Reg<intctrl::INTCTRL_SPEC>;
#[doc = "Interrupt Control"]
pub mod intctrl;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "MUXPOS register accessor: an alias for `Reg<MUXPOS_SPEC>`"]
pub type MUXPOS = crate::Reg<muxpos::MUXPOS_SPEC>;
#[doc = "Positive mux input"]
pub mod muxpos;
#[doc = "RES register accessor: an alias for `Reg<RES_SPEC>`"]
pub type RES = crate::Reg<res::RES_SPEC>;
#[doc = "ADC Accumulator Result"]
pub mod res;
#[doc = "SAMPCTRL register accessor: an alias for `Reg<SAMPCTRL_SPEC>`"]
pub type SAMPCTRL = crate::Reg<sampctrl::SAMPCTRL_SPEC>;
#[doc = "Sample Control"]
pub mod sampctrl;
#[doc = "TEMP register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Temporary Data"]
pub mod temp;
#[doc = "WINHT register accessor: an alias for `Reg<WINHT_SPEC>`"]
pub type WINHT = crate::Reg<winht::WINHT_SPEC>;
#[doc = "Window comparator high threshold"]
pub mod winht;
#[doc = "WINLT register accessor: an alias for `Reg<WINLT_SPEC>`"]
pub type WINLT = crate::Reg<winlt::WINLT_SPEC>;
#[doc = "Window comparator low threshold"]
pub mod winlt;
