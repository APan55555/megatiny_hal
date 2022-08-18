#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCLK Control A"]
    pub mclkctrla: crate::attiny412pac::Reg<mclkctrla::MCLKCTRLA_SPEC>,
    #[doc = "0x01 - MCLK Control B"]
    pub mclkctrlb: crate::attiny412pac::Reg<mclkctrlb::MCLKCTRLB_SPEC>,
    #[doc = "0x02 - MCLK Lock"]
    pub mclklock: crate::attiny412pac::Reg<mclklock::MCLKLOCK_SPEC>,
    #[doc = "0x03 - MCLK Status"]
    pub mclkstatus: crate::attiny412pac::Reg<mclkstatus::MCLKSTATUS_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x10 - OSC20M Control A"]
    pub osc20mctrla: crate::attiny412pac::Reg<osc20mctrla::OSC20MCTRLA_SPEC>,
    #[doc = "0x11 - OSC20M Calibration A"]
    pub osc20mcaliba: crate::attiny412pac::Reg<osc20mcaliba::OSC20MCALIBA_SPEC>,
    #[doc = "0x12 - OSC20M Calibration B"]
    pub osc20mcalibb: crate::attiny412pac::Reg<osc20mcalibb::OSC20MCALIBB_SPEC>,
    _reserved7: [u8; 0x05],
    #[doc = "0x18 - OSC32K Control A"]
    pub osc32kctrla: crate::attiny412pac::Reg<osc32kctrla::OSC32KCTRLA_SPEC>,
}
#[doc = "MCLKCTRLA register accessor: an alias for `Reg<MCLKCTRLA_SPEC>`"]
pub type MCLKCTRLA = crate::attiny412pac::Reg<mclkctrla::MCLKCTRLA_SPEC>;
#[doc = "MCLK Control A"]
pub mod mclkctrla;
#[doc = "MCLKCTRLB register accessor: an alias for `Reg<MCLKCTRLB_SPEC>`"]
pub type MCLKCTRLB = crate::attiny412pac::Reg<mclkctrlb::MCLKCTRLB_SPEC>;
#[doc = "MCLK Control B"]
pub mod mclkctrlb;
#[doc = "MCLKLOCK register accessor: an alias for `Reg<MCLKLOCK_SPEC>`"]
pub type MCLKLOCK = crate::attiny412pac::Reg<mclklock::MCLKLOCK_SPEC>;
#[doc = "MCLK Lock"]
pub mod mclklock;
#[doc = "MCLKSTATUS register accessor: an alias for `Reg<MCLKSTATUS_SPEC>`"]
pub type MCLKSTATUS = crate::attiny412pac::Reg<mclkstatus::MCLKSTATUS_SPEC>;
#[doc = "MCLK Status"]
pub mod mclkstatus;
#[doc = "OSC20MCALIBA register accessor: an alias for `Reg<OSC20MCALIBA_SPEC>`"]
pub type OSC20MCALIBA = crate::attiny412pac::Reg<osc20mcaliba::OSC20MCALIBA_SPEC>;
#[doc = "OSC20M Calibration A"]
pub mod osc20mcaliba;
#[doc = "OSC20MCALIBB register accessor: an alias for `Reg<OSC20MCALIBB_SPEC>`"]
pub type OSC20MCALIBB = crate::attiny412pac::Reg<osc20mcalibb::OSC20MCALIBB_SPEC>;
#[doc = "OSC20M Calibration B"]
pub mod osc20mcalibb;
#[doc = "OSC20MCTRLA register accessor: an alias for `Reg<OSC20MCTRLA_SPEC>`"]
pub type OSC20MCTRLA = crate::attiny412pac::Reg<osc20mctrla::OSC20MCTRLA_SPEC>;
#[doc = "OSC20M Control A"]
pub mod osc20mctrla;
#[doc = "OSC32KCTRLA register accessor: an alias for `Reg<OSC32KCTRLA_SPEC>`"]
pub type OSC32KCTRLA = crate::attiny412pac::Reg<osc32kctrla::OSC32KCTRLA_SPEC>;
#[doc = "OSC32K Control A"]
pub mod osc32kctrla;
