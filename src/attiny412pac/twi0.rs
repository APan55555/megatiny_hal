#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control A"]
    pub ctrla: crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>,
    _reserved1: [u8; 0x01],
    #[doc = "0x02 - Debug Control Register"]
    pub dbgctrl: crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x03 - Master Control A"]
    pub mctrla: crate::attiny412pac::Reg<mctrla::MCTRLA_SPEC>,
    #[doc = "0x04 - Master Control B"]
    pub mctrlb: crate::attiny412pac::Reg<mctrlb::MCTRLB_SPEC>,
    #[doc = "0x05 - Master Status"]
    pub mstatus: crate::attiny412pac::Reg<mstatus::MSTATUS_SPEC>,
    #[doc = "0x06 - Master Baurd Rate Control"]
    pub mbaud: crate::attiny412pac::Reg<mbaud::MBAUD_SPEC>,
    #[doc = "0x07 - Master Address"]
    pub maddr: crate::attiny412pac::Reg<maddr::MADDR_SPEC>,
    #[doc = "0x08 - Master Data"]
    pub mdata: crate::attiny412pac::Reg<mdata::MDATA_SPEC>,
    #[doc = "0x09 - Slave Control A"]
    pub sctrla: crate::attiny412pac::Reg<sctrla::SCTRLA_SPEC>,
    #[doc = "0x0a - Slave Control B"]
    pub sctrlb: crate::attiny412pac::Reg<sctrlb::SCTRLB_SPEC>,
    #[doc = "0x0b - Slave Status"]
    pub sstatus: crate::attiny412pac::Reg<sstatus::SSTATUS_SPEC>,
    #[doc = "0x0c - Slave Address"]
    pub saddr: crate::attiny412pac::Reg<saddr::SADDR_SPEC>,
    #[doc = "0x0d - Slave Data"]
    pub sdata: crate::attiny412pac::Reg<sdata::SDATA_SPEC>,
    #[doc = "0x0e - Slave Address Mask"]
    pub saddrmask: crate::attiny412pac::Reg<saddrmask::SADDRMASK_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::attiny412pac::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Control A"]
pub mod ctrla;
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::attiny412pac::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control Register"]
pub mod dbgctrl;
#[doc = "MADDR register accessor: an alias for `Reg<MADDR_SPEC>`"]
pub type MADDR = crate::attiny412pac::Reg<maddr::MADDR_SPEC>;
#[doc = "Master Address"]
pub mod maddr;
#[doc = "MBAUD register accessor: an alias for `Reg<MBAUD_SPEC>`"]
pub type MBAUD = crate::attiny412pac::Reg<mbaud::MBAUD_SPEC>;
#[doc = "Master Baurd Rate Control"]
pub mod mbaud;
#[doc = "MCTRLA register accessor: an alias for `Reg<MCTRLA_SPEC>`"]
pub type MCTRLA = crate::attiny412pac::Reg<mctrla::MCTRLA_SPEC>;
#[doc = "Master Control A"]
pub mod mctrla;
#[doc = "MCTRLB register accessor: an alias for `Reg<MCTRLB_SPEC>`"]
pub type MCTRLB = crate::attiny412pac::Reg<mctrlb::MCTRLB_SPEC>;
#[doc = "Master Control B"]
pub mod mctrlb;
#[doc = "MDATA register accessor: an alias for `Reg<MDATA_SPEC>`"]
pub type MDATA = crate::attiny412pac::Reg<mdata::MDATA_SPEC>;
#[doc = "Master Data"]
pub mod mdata;
#[doc = "MSTATUS register accessor: an alias for `Reg<MSTATUS_SPEC>`"]
pub type MSTATUS = crate::attiny412pac::Reg<mstatus::MSTATUS_SPEC>;
#[doc = "Master Status"]
pub mod mstatus;
#[doc = "SADDR register accessor: an alias for `Reg<SADDR_SPEC>`"]
pub type SADDR = crate::attiny412pac::Reg<saddr::SADDR_SPEC>;
#[doc = "Slave Address"]
pub mod saddr;
#[doc = "SADDRMASK register accessor: an alias for `Reg<SADDRMASK_SPEC>`"]
pub type SADDRMASK = crate::attiny412pac::Reg<saddrmask::SADDRMASK_SPEC>;
#[doc = "Slave Address Mask"]
pub mod saddrmask;
#[doc = "SCTRLA register accessor: an alias for `Reg<SCTRLA_SPEC>`"]
pub type SCTRLA = crate::attiny412pac::Reg<sctrla::SCTRLA_SPEC>;
#[doc = "Slave Control A"]
pub mod sctrla;
#[doc = "SCTRLB register accessor: an alias for `Reg<SCTRLB_SPEC>`"]
pub type SCTRLB = crate::attiny412pac::Reg<sctrlb::SCTRLB_SPEC>;
#[doc = "Slave Control B"]
pub mod sctrlb;
#[doc = "SDATA register accessor: an alias for `Reg<SDATA_SPEC>`"]
pub type SDATA = crate::attiny412pac::Reg<sdata::SDATA_SPEC>;
#[doc = "Slave Data"]
pub mod sdata;
#[doc = "SSTATUS register accessor: an alias for `Reg<SSTATUS_SPEC>`"]
pub type SSTATUS = crate::attiny412pac::Reg<sstatus::SSTATUS_SPEC>;
#[doc = "Slave Status"]
pub mod sstatus;
