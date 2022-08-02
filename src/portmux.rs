#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port Multiplexer Control A"]
    pub ctrla: crate::Reg<ctrla::CTRLA_SPEC>,
    #[doc = "0x01 - Port Multiplexer Control B"]
    pub ctrlb: crate::Reg<ctrlb::CTRLB_SPEC>,
    #[doc = "0x02 - Port Multiplexer Control C"]
    pub ctrlc: crate::Reg<ctrlc::CTRLC_SPEC>,
    #[doc = "0x03 - Port Multiplexer Control D"]
    pub ctrld: crate::Reg<ctrld::CTRLD_SPEC>,
}
#[doc = "CTRLA register accessor: an alias for `Reg<CTRLA_SPEC>`"]
pub type CTRLA = crate::Reg<ctrla::CTRLA_SPEC>;
#[doc = "Port Multiplexer Control A"]
pub mod ctrla;
#[doc = "CTRLB register accessor: an alias for `Reg<CTRLB_SPEC>`"]
pub type CTRLB = crate::Reg<ctrlb::CTRLB_SPEC>;
#[doc = "Port Multiplexer Control B"]
pub mod ctrlb;
#[doc = "CTRLC register accessor: an alias for `Reg<CTRLC_SPEC>`"]
pub type CTRLC = crate::Reg<ctrlc::CTRLC_SPEC>;
#[doc = "Port Multiplexer Control C"]
pub mod ctrlc;
#[doc = "CTRLD register accessor: an alias for `Reg<CTRLD_SPEC>`"]
pub type CTRLD = crate::Reg<ctrld::CTRLD_SPEC>;
#[doc = "Port Multiplexer Control D"]
pub mod ctrld;
