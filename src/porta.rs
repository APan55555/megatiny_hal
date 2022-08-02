#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Direction"]
    pub dir: crate::Reg<dir::DIR_SPEC>,
    #[doc = "0x01 - Data Direction Set"]
    pub dirset: crate::Reg<dirset::DIRSET_SPEC>,
    #[doc = "0x02 - Data Direction Clear"]
    pub dirclr: crate::Reg<dirclr::DIRCLR_SPEC>,
    #[doc = "0x03 - Data Direction Toggle"]
    pub dirtgl: crate::Reg<dirtgl::DIRTGL_SPEC>,
    #[doc = "0x04 - Output Value"]
    pub out: crate::Reg<out::OUT_SPEC>,
    #[doc = "0x05 - Output Value Set"]
    pub outset: crate::Reg<outset::OUTSET_SPEC>,
    #[doc = "0x06 - Output Value Clear"]
    pub outclr: crate::Reg<outclr::OUTCLR_SPEC>,
    #[doc = "0x07 - Output Value Toggle"]
    pub outtgl: crate::Reg<outtgl::OUTTGL_SPEC>,
    #[doc = "0x08 - Input Value"]
    pub in_: crate::Reg<in_::IN_SPEC>,
    #[doc = "0x09 - Interrupt Flags"]
    pub intflags: crate::Reg<intflags::INTFLAGS_SPEC>,
    _reserved10: [u8; 0x06],
    #[doc = "0x10 - Pin 0 Control"]
    pub pin0ctrl: crate::Reg<pin0ctrl::PIN0CTRL_SPEC>,
    #[doc = "0x11 - Pin 1 Control"]
    pub pin1ctrl: crate::Reg<pin1ctrl::PIN1CTRL_SPEC>,
    #[doc = "0x12 - Pin 2 Control"]
    pub pin2ctrl: crate::Reg<pin2ctrl::PIN2CTRL_SPEC>,
    #[doc = "0x13 - Pin 3 Control"]
    pub pin3ctrl: crate::Reg<pin3ctrl::PIN3CTRL_SPEC>,
    #[doc = "0x14 - Pin 4 Control"]
    pub pin4ctrl: crate::Reg<pin4ctrl::PIN4CTRL_SPEC>,
    #[doc = "0x15 - Pin 5 Control"]
    pub pin5ctrl: crate::Reg<pin5ctrl::PIN5CTRL_SPEC>,
    #[doc = "0x16 - Pin 6 Control"]
    pub pin6ctrl: crate::Reg<pin6ctrl::PIN6CTRL_SPEC>,
    #[doc = "0x17 - Pin 7 Control"]
    pub pin7ctrl: crate::Reg<pin7ctrl::PIN7CTRL_SPEC>,
}
#[doc = "DIR register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Data Direction"]
pub mod dir;
#[doc = "DIRCLR register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Data Direction Clear"]
pub mod dirclr;
#[doc = "DIRSET register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Data Direction Set"]
pub mod dirset;
#[doc = "DIRTGL register accessor: an alias for `Reg<DIRTGL_SPEC>`"]
pub type DIRTGL = crate::Reg<dirtgl::DIRTGL_SPEC>;
#[doc = "Data Direction Toggle"]
pub mod dirtgl;
#[doc = "IN register accessor: an alias for `Reg<IN_SPEC>`"]
pub type IN = crate::Reg<in_::IN_SPEC>;
#[doc = "Input Value"]
pub mod in_;
#[doc = "INTFLAGS register accessor: an alias for `Reg<INTFLAGS_SPEC>`"]
pub type INTFLAGS = crate::Reg<intflags::INTFLAGS_SPEC>;
#[doc = "Interrupt Flags"]
pub mod intflags;
#[doc = "OUT register accessor: an alias for `Reg<OUT_SPEC>`"]
pub type OUT = crate::Reg<out::OUT_SPEC>;
#[doc = "Output Value"]
pub mod out;
#[doc = "OUTCLR register accessor: an alias for `Reg<OUTCLR_SPEC>`"]
pub type OUTCLR = crate::Reg<outclr::OUTCLR_SPEC>;
#[doc = "Output Value Clear"]
pub mod outclr;
#[doc = "OUTSET register accessor: an alias for `Reg<OUTSET_SPEC>`"]
pub type OUTSET = crate::Reg<outset::OUTSET_SPEC>;
#[doc = "Output Value Set"]
pub mod outset;
#[doc = "OUTTGL register accessor: an alias for `Reg<OUTTGL_SPEC>`"]
pub type OUTTGL = crate::Reg<outtgl::OUTTGL_SPEC>;
#[doc = "Output Value Toggle"]
pub mod outtgl;
#[doc = "PIN0CTRL register accessor: an alias for `Reg<PIN0CTRL_SPEC>`"]
pub type PIN0CTRL = crate::Reg<pin0ctrl::PIN0CTRL_SPEC>;
#[doc = "Pin 0 Control"]
pub mod pin0ctrl;
#[doc = "PIN1CTRL register accessor: an alias for `Reg<PIN1CTRL_SPEC>`"]
pub type PIN1CTRL = crate::Reg<pin1ctrl::PIN1CTRL_SPEC>;
#[doc = "Pin 1 Control"]
pub mod pin1ctrl;
#[doc = "PIN2CTRL register accessor: an alias for `Reg<PIN2CTRL_SPEC>`"]
pub type PIN2CTRL = crate::Reg<pin2ctrl::PIN2CTRL_SPEC>;
#[doc = "Pin 2 Control"]
pub mod pin2ctrl;
#[doc = "PIN3CTRL register accessor: an alias for `Reg<PIN3CTRL_SPEC>`"]
pub type PIN3CTRL = crate::Reg<pin3ctrl::PIN3CTRL_SPEC>;
#[doc = "Pin 3 Control"]
pub mod pin3ctrl;
#[doc = "PIN4CTRL register accessor: an alias for `Reg<PIN4CTRL_SPEC>`"]
pub type PIN4CTRL = crate::Reg<pin4ctrl::PIN4CTRL_SPEC>;
#[doc = "Pin 4 Control"]
pub mod pin4ctrl;
#[doc = "PIN5CTRL register accessor: an alias for `Reg<PIN5CTRL_SPEC>`"]
pub type PIN5CTRL = crate::Reg<pin5ctrl::PIN5CTRL_SPEC>;
#[doc = "Pin 5 Control"]
pub mod pin5ctrl;
#[doc = "PIN6CTRL register accessor: an alias for `Reg<PIN6CTRL_SPEC>`"]
pub type PIN6CTRL = crate::Reg<pin6ctrl::PIN6CTRL_SPEC>;
#[doc = "Pin 6 Control"]
pub mod pin6ctrl;
#[doc = "PIN7CTRL register accessor: an alias for `Reg<PIN7CTRL_SPEC>`"]
pub type PIN7CTRL = crate::Reg<pin7ctrl::PIN7CTRL_SPEC>;
#[doc = "Pin 7 Control"]
pub mod pin7ctrl;
