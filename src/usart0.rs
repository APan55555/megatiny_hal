#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Receive Data Low Byte"]
    pub rxdatal: crate::Reg<rxdatal::RXDATAL_SPEC>,
    #[doc = "0x01 - Receive Data High Byte"]
    pub rxdatah: crate::Reg<rxdatah::RXDATAH_SPEC>,
    #[doc = "0x02 - Transmit Data Low Byte"]
    pub txdatal: crate::Reg<txdatal::TXDATAL_SPEC>,
    #[doc = "0x03 - Transmit Data High Byte"]
    pub txdatah: crate::Reg<txdatah::TXDATAH_SPEC>,
    #[doc = "0x04 - Status"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x05 - Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x06 - Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x07 - Control C"]
    pub ctrlc: crate::Reg<ctrlc::CTRLC_SPEC>,
    #[doc = "0x08 - Baud Rate"]
    pub baud: crate::Reg<baud::BAUD_SPEC>,
    _reserved9: [u8; 0x01],
    #[doc = "0x0b - Debug Control"]
    pub dbgctrl: crate::Reg<dbgctrl::DBGCTRL_SPEC>,
    #[doc = "0x0c - Event Control"]
    pub evctrl: crate::Reg<evctrl::EVCTRL_SPEC>,
    #[doc = "0x0d - IRCOM Transmitter Pulse Length Control"]
    pub txplctrl: crate::Reg<txplctrl::TXPLCTRL_SPEC>,
    #[doc = "0x0e - IRCOM Receiver Pulse Length Control"]
    pub rxplctrl: crate::Reg<rxplctrl::RXPLCTRL_SPEC>,
}
#[doc = "BAUD register accessor: an alias for `Reg<BAUD_SPEC>`"]
pub type BAUD = crate::Reg<baud::BAUD_SPEC>;
#[doc = "Baud Rate"]
pub mod baud;
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
#[doc = "DBGCTRL register accessor: an alias for `Reg<DBGCTRL_SPEC>`"]
pub type DBGCTRL = crate::Reg<dbgctrl::DBGCTRL_SPEC>;
#[doc = "Debug Control"]
pub mod dbgctrl;
#[doc = "EVCTRL register accessor: an alias for `Reg<EVCTRL_SPEC>`"]
pub type EVCTRL = crate::Reg<evctrl::EVCTRL_SPEC>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "RXDATAH register accessor: an alias for `Reg<RXDATAH_SPEC>`"]
pub type RXDATAH = crate::Reg<rxdatah::RXDATAH_SPEC>;
#[doc = "Receive Data High Byte"]
pub mod rxdatah;
#[doc = "RXDATAL register accessor: an alias for `Reg<RXDATAL_SPEC>`"]
pub type RXDATAL = crate::Reg<rxdatal::RXDATAL_SPEC>;
#[doc = "Receive Data Low Byte"]
pub mod rxdatal;
#[doc = "RXPLCTRL register accessor: an alias for `Reg<RXPLCTRL_SPEC>`"]
pub type RXPLCTRL = crate::Reg<rxplctrl::RXPLCTRL_SPEC>;
#[doc = "IRCOM Receiver Pulse Length Control"]
pub mod rxplctrl;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "TXDATAH register accessor: an alias for `Reg<TXDATAH_SPEC>`"]
pub type TXDATAH = crate::Reg<txdatah::TXDATAH_SPEC>;
#[doc = "Transmit Data High Byte"]
pub mod txdatah;
#[doc = "TXDATAL register accessor: an alias for `Reg<TXDATAL_SPEC>`"]
pub type TXDATAL = crate::Reg<txdatal::TXDATAL_SPEC>;
#[doc = "Transmit Data Low Byte"]
pub mod txdatal;
#[doc = "TXPLCTRL register accessor: an alias for `Reg<TXPLCTRL_SPEC>`"]
pub type TXPLCTRL = crate::Reg<txplctrl::TXPLCTRL_SPEC>;
#[doc = "IRCOM Transmitter Pulse Length Control"]
pub mod txplctrl;
