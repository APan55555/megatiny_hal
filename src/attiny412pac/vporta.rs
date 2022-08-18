#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir: crate::attiny412pac::Reg<dir::DIR_SPEC>,
    #[doc = "0x01 - Output Value"]
    pub out: crate::attiny412pac::Reg<out::OUT_SPEC>,
    #[doc = "0x02 - Input Value"]
    pub in_: crate::attiny412pac::Reg<in_::IN_SPEC>,
    #[doc = "0x03 - Interrupt Flags"]
    pub intflags: crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>,
}
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::attiny412pac::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::attiny412pac::Reg<in_::IN_SPEC>;
#[doc = "Input Value"]
pub mod in_;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::attiny412pac::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::attiny412pac::Reg<out::OUT_SPEC>;
#[doc = "Output Value"]
pub mod out;
