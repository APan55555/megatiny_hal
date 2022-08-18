#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Configuration"]
    pub wdtcfg: crate::attiny412pac::Reg<wdtcfg::WDTCFG_SPEC>,
    #[doc = "0x01 - BOD Configuration"]
    pub bodcfg: crate::attiny412pac::Reg<bodcfg::BODCFG_SPEC>,
    #[doc = "0x02 - Oscillator Configuration"]
    pub osccfg: crate::attiny412pac::Reg<osccfg::OSCCFG_SPEC>,
    _reserved3: [u8; 0x01],
    #[doc = "0x04 - TCD0 Configuration"]
    pub tcd0cfg: crate::attiny412pac::Reg<tcd0cfg::TCD0CFG_SPEC>,
    #[doc = "0x05 - System Configuration 0"]
    pub syscfg0: crate::attiny412pac::Reg<syscfg0::SYSCFG0_SPEC>,
    #[doc = "0x06 - System Configuration 1"]
    pub syscfg1: crate::attiny412pac::Reg<syscfg1::SYSCFG1_SPEC>,
    #[doc = "0x07 - Application Code Section End"]
    pub append: crate::attiny412pac::Reg<append::APPEND_SPEC>,
    #[doc = "0x08 - Boot Section End"]
    pub bootend: crate::attiny412pac::Reg<bootend::BOOTEND_SPEC>,
}
#[doc = "APPEND register accessor: an alias for `Reg<APPEND_SPEC>`"]
pub type APPEND = crate::attiny412pac::Reg<append::APPEND_SPEC>;
#[doc = "Application Code Section End"]
pub mod append;
#[doc = "BODCFG register accessor: an alias for `Reg<BODCFG_SPEC>`"]
pub type BODCFG = crate::attiny412pac::Reg<bodcfg::BODCFG_SPEC>;
#[doc = "BOD Configuration"]
pub mod bodcfg;
#[doc = "BOOTEND register accessor: an alias for `Reg<BOOTEND_SPEC>`"]
pub type BOOTEND = crate::attiny412pac::Reg<bootend::BOOTEND_SPEC>;
#[doc = "Boot Section End"]
pub mod bootend;
#[doc = "OSCCFG register accessor: an alias for `Reg<OSCCFG_SPEC>`"]
pub type OSCCFG = crate::attiny412pac::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration"]
pub mod osccfg;
#[doc = "SYSCFG0 register accessor: an alias for `Reg<SYSCFG0_SPEC>`"]
pub type SYSCFG0 = crate::attiny412pac::Reg<syscfg0::SYSCFG0_SPEC>;
#[doc = "System Configuration 0"]
pub mod syscfg0;
#[doc = "SYSCFG1 register accessor: an alias for `Reg<SYSCFG1_SPEC>`"]
pub type SYSCFG1 = crate::attiny412pac::Reg<syscfg1::SYSCFG1_SPEC>;
#[doc = "System Configuration 1"]
pub mod syscfg1;
#[doc = "TCD0CFG register accessor: an alias for `Reg<TCD0CFG_SPEC>`"]
pub type TCD0CFG = crate::attiny412pac::Reg<tcd0cfg::TCD0CFG_SPEC>;
#[doc = "TCD0 Configuration"]
pub mod tcd0cfg;
#[doc = "WDTCFG register accessor: an alias for `Reg<WDTCFG_SPEC>`"]
pub type WDTCFG = crate::attiny412pac::Reg<wdtcfg::WDTCFG_SPEC>;
#[doc = "Watchdog Configuration"]
pub mod wdtcfg;
