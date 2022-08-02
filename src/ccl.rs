#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Sequential Control 0"]
    pub seqctrl0: crate::Reg<seqctrl0::SEQCTRL0_SPEC>,
    _reserved2: [u8; 0x03],
    #[doc = "0x05 - LUT Control 0 A"]
    pub lut0ctrla: crate::Reg<lut0ctrla::LUT0CTRLA_SPEC>,
    #[doc = "0x06 - LUT Control 0 B"]
    pub lut0ctrlb: crate::Reg<lut0ctrlb::LUT0CTRLB_SPEC>,
    #[doc = "0x07 - LUT Control 0 C"]
    pub lut0ctrlc: crate::Reg<lut0ctrlc::LUT0CTRLC_SPEC>,
    #[doc = "0x08 - Truth 0"]
    pub truth0: crate::Reg<truth0::TRUTH0_SPEC>,
    #[doc = "0x09 - LUT Control 1 A"]
    pub lut1ctrla: crate::Reg<lut1ctrla::LUT1CTRLA_SPEC>,
    #[doc = "0x0a - LUT Control 1 B"]
    pub lut1ctrlb: crate::Reg<lut1ctrlb::LUT1CTRLB_SPEC>,
    #[doc = "0x0b - LUT Control 1 C"]
    pub lut1ctrlc: crate::Reg<lut1ctrlc::LUT1CTRLC_SPEC>,
    #[doc = "0x0c - Truth 1"]
    pub truth1: crate::Reg<truth1::TRUTH1_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control Register A"]
pub mod ctrla;
#[doc = "LUT0CTRLA register accessor: an alias for `Reg<LUT0CTRLA_SPEC>`"]
pub type LUT0CTRLA = crate::Reg<lut0ctrla::LUT0CTRLA_SPEC>;
#[doc = "LUT Control 0 A"]
pub mod lut0ctrla;
#[doc = "LUT0CTRLB register accessor: an alias for `Reg<LUT0CTRLB_SPEC>`"]
pub type LUT0CTRLB = crate::Reg<lut0ctrlb::LUT0CTRLB_SPEC>;
#[doc = "LUT Control 0 B"]
pub mod lut0ctrlb;
#[doc = "LUT0CTRLC register accessor: an alias for `Reg<LUT0CTRLC_SPEC>`"]
pub type LUT0CTRLC = crate::Reg<lut0ctrlc::LUT0CTRLC_SPEC>;
#[doc = "LUT Control 0 C"]
pub mod lut0ctrlc;
#[doc = "LUT1CTRLA register accessor: an alias for `Reg<LUT1CTRLA_SPEC>`"]
pub type LUT1CTRLA = crate::Reg<lut1ctrla::LUT1CTRLA_SPEC>;
#[doc = "LUT Control 1 A"]
pub mod lut1ctrla;
#[doc = "LUT1CTRLB register accessor: an alias for `Reg<LUT1CTRLB_SPEC>`"]
pub type LUT1CTRLB = crate::Reg<lut1ctrlb::LUT1CTRLB_SPEC>;
#[doc = "LUT Control 1 B"]
pub mod lut1ctrlb;
#[doc = "LUT1CTRLC register accessor: an alias for `Reg<LUT1CTRLC_SPEC>`"]
pub type LUT1CTRLC = crate::Reg<lut1ctrlc::LUT1CTRLC_SPEC>;
#[doc = "LUT Control 1 C"]
pub mod lut1ctrlc;
#[doc = "SEQCTRL0 register accessor: an alias for `Reg<SEQCTRL0_SPEC>`"]
pub type SEQCTRL0 = crate::Reg<seqctrl0::SEQCTRL0_SPEC>;
#[doc = "Sequential Control 0"]
pub mod seqctrl0;
#[doc = "TRUTH0 register accessor: an alias for `Reg<TRUTH0_SPEC>`"]
pub type TRUTH0 = crate::Reg<truth0::TRUTH0_SPEC>;
#[doc = "Truth 0"]
pub mod truth0;
#[doc = "TRUTH1 register accessor: an alias for `Reg<TRUTH1_SPEC>`"]
pub type TRUTH1 = crate::Reg<truth1::TRUTH1_SPEC>;
#[doc = "Truth 1"]
pub mod truth1;
