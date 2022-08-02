#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Asynchronous Channel Strobe"]
    pub asyncstrobe: crate::Reg<asyncstrobe::ASYNCSTROBE_SPEC>,
    #[doc = "0x01 - Synchronous Channel Strobe"]
    pub syncstrobe: crate::Reg<syncstrobe::SYNCSTROBE_SPEC>,
    #[doc = "0x02 - Asynchronous Channel 0 Generator Selection"]
    pub asyncch0: crate::Reg<asyncch0::ASYNCCH0_SPEC>,
    #[doc = "0x03 - Asynchronous Channel 1 Generator Selection"]
    pub asyncch1: crate::Reg<asyncch1::ASYNCCH1_SPEC>,
    #[doc = "0x04 - Asynchronous Channel 2 Generator Selection"]
    pub asyncch2: crate::Reg<asyncch2::ASYNCCH2_SPEC>,
    #[doc = "0x05 - Asynchronous Channel 3 Generator Selection"]
    pub asyncch3: crate::Reg<asyncch3::ASYNCCH3_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x0a - Synchronous Channel 0 Generator Selection"]
    pub syncch0: crate::Reg<syncch0::SYNCCH0_SPEC>,
    #[doc = "0x0b - Synchronous Channel 1 Generator Selection"]
    pub syncch1: crate::Reg<syncch1::SYNCCH1_SPEC>,
    _reserved8: [u8; 0x06],
    #[doc = "0x12 - Asynchronous User Ch 0 Input Selection - TCB0"]
    pub asyncuser0: crate::Reg<asyncuser0::ASYNCUSER0_SPEC>,
    #[doc = "0x13 - Asynchronous User Ch 1 Input Selection - ADC0"]
    pub asyncuser1: crate::Reg<asyncuser1::ASYNCUSER1_SPEC>,
    #[doc = "0x14 - Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
    pub asyncuser2: crate::Reg<asyncuser2::ASYNCUSER2_SPEC>,
    #[doc = "0x15 - Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
    pub asyncuser3: crate::Reg<asyncuser3::ASYNCUSER3_SPEC>,
    #[doc = "0x16 - Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1"]
    pub asyncuser4: crate::Reg<asyncuser4::ASYNCUSER4_SPEC>,
    #[doc = "0x17 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    pub asyncuser5: crate::Reg<asyncuser5::ASYNCUSER5_SPEC>,
    #[doc = "0x18 - Asynchronous User Ch 6 Input Selection - TCD0 Event 0"]
    pub asyncuser6: crate::Reg<asyncuser6::ASYNCUSER6_SPEC>,
    #[doc = "0x19 - Asynchronous User Ch 7 Input Selection - TCD0 Event 1"]
    pub asyncuser7: crate::Reg<asyncuser7::ASYNCUSER7_SPEC>,
    #[doc = "0x1a - Asynchronous User Ch 8 Input Selection - Event Out 0"]
    pub asyncuser8: crate::Reg<asyncuser8::ASYNCUSER8_SPEC>,
    #[doc = "0x1b - Asynchronous User Ch 9 Input Selection - Event Out 1"]
    pub asyncuser9: crate::Reg<asyncuser9::ASYNCUSER9_SPEC>,
    #[doc = "0x1c - Asynchronous User Ch 10 Input Selection - Event Out 2"]
    pub asyncuser10: crate::Reg<asyncuser10::ASYNCUSER10_SPEC>,
    _reserved19: [u8; 0x05],
    #[doc = "0x22 - Synchronous User Ch 0 Input Selection - TCA0"]
    pub syncuser0: crate::Reg<syncuser0::SYNCUSER0_SPEC>,
    #[doc = "0x23 - Synchronous User Ch 1 Input Selection - USART0"]
    pub syncuser1: crate::Reg<syncuser1::SYNCUSER1_SPEC>,
}
#[doc = "ASYNCCH0 register accessor: an alias for `Reg<ASYNCCH0_SPEC>`"]
pub type ASYNCCH0 = crate::Reg<asyncch0::ASYNCCH0_SPEC>;
#[doc = "Asynchronous Channel 0 Generator Selection"]
pub mod asyncch0;
#[doc = "ASYNCCH1 register accessor: an alias for `Reg<ASYNCCH1_SPEC>`"]
pub type ASYNCCH1 = crate::Reg<asyncch1::ASYNCCH1_SPEC>;
#[doc = "Asynchronous Channel 1 Generator Selection"]
pub mod asyncch1;
#[doc = "ASYNCCH2 register accessor: an alias for `Reg<ASYNCCH2_SPEC>`"]
pub type ASYNCCH2 = crate::Reg<asyncch2::ASYNCCH2_SPEC>;
#[doc = "Asynchronous Channel 2 Generator Selection"]
pub mod asyncch2;
#[doc = "ASYNCCH3 register accessor: an alias for `Reg<ASYNCCH3_SPEC>`"]
pub type ASYNCCH3 = crate::Reg<asyncch3::ASYNCCH3_SPEC>;
#[doc = "Asynchronous Channel 3 Generator Selection"]
pub mod asyncch3;
#[doc = "ASYNCSTROBE register accessor: an alias for `Reg<ASYNCSTROBE_SPEC>`"]
pub type ASYNCSTROBE = crate::Reg<asyncstrobe::ASYNCSTROBE_SPEC>;
#[doc = "Asynchronous Channel Strobe"]
pub mod asyncstrobe;
#[doc = "ASYNCUSER0 register accessor: an alias for `Reg<ASYNCUSER0_SPEC>`"]
pub type ASYNCUSER0 = crate::Reg<asyncuser0::ASYNCUSER0_SPEC>;
#[doc = "Asynchronous User Ch 0 Input Selection - TCB0"]
pub mod asyncuser0;
#[doc = "ASYNCUSER1 register accessor: an alias for `Reg<ASYNCUSER1_SPEC>`"]
pub type ASYNCUSER1 = crate::Reg<asyncuser1::ASYNCUSER1_SPEC>;
#[doc = "Asynchronous User Ch 1 Input Selection - ADC0"]
pub mod asyncuser1;
#[doc = "ASYNCUSER10 register accessor: an alias for `Reg<ASYNCUSER10_SPEC>`"]
pub type ASYNCUSER10 = crate::Reg<asyncuser10::ASYNCUSER10_SPEC>;
#[doc = "Asynchronous User Ch 10 Input Selection - Event Out 2"]
pub mod asyncuser10;
#[doc = "ASYNCUSER2 register accessor: an alias for `Reg<ASYNCUSER2_SPEC>`"]
pub type ASYNCUSER2 = crate::Reg<asyncuser2::ASYNCUSER2_SPEC>;
#[doc = "Asynchronous User Ch 2 Input Selection - CCL LUT0 Event 0"]
pub mod asyncuser2;
#[doc = "ASYNCUSER3 register accessor: an alias for `Reg<ASYNCUSER3_SPEC>`"]
pub type ASYNCUSER3 = crate::Reg<asyncuser3::ASYNCUSER3_SPEC>;
#[doc = "Asynchronous User Ch 3 Input Selection - CCL LUT1 Event 0"]
pub mod asyncuser3;
#[doc = "ASYNCUSER4 register accessor: an alias for `Reg<ASYNCUSER4_SPEC>`"]
pub type ASYNCUSER4 = crate::Reg<asyncuser4::ASYNCUSER4_SPEC>;
#[doc = "Asynchronous User Ch 4 Input Selection - CCL LUT0 Event 1"]
pub mod asyncuser4;
#[doc = "ASYNCUSER5 register accessor: an alias for `Reg<ASYNCUSER5_SPEC>`"]
pub type ASYNCUSER5 = crate::Reg<asyncuser5::ASYNCUSER5_SPEC>;
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub mod asyncuser5;
#[doc = "ASYNCUSER6 register accessor: an alias for `Reg<ASYNCUSER6_SPEC>`"]
pub type ASYNCUSER6 = crate::Reg<asyncuser6::ASYNCUSER6_SPEC>;
#[doc = "Asynchronous User Ch 6 Input Selection - TCD0 Event 0"]
pub mod asyncuser6;
#[doc = "ASYNCUSER7 register accessor: an alias for `Reg<ASYNCUSER7_SPEC>`"]
pub type ASYNCUSER7 = crate::Reg<asyncuser7::ASYNCUSER7_SPEC>;
#[doc = "Asynchronous User Ch 7 Input Selection - TCD0 Event 1"]
pub mod asyncuser7;
#[doc = "ASYNCUSER8 register accessor: an alias for `Reg<ASYNCUSER8_SPEC>`"]
pub type ASYNCUSER8 = crate::Reg<asyncuser8::ASYNCUSER8_SPEC>;
#[doc = "Asynchronous User Ch 8 Input Selection - Event Out 0"]
pub mod asyncuser8;
#[doc = "ASYNCUSER9 register accessor: an alias for `Reg<ASYNCUSER9_SPEC>`"]
pub type ASYNCUSER9 = crate::Reg<asyncuser9::ASYNCUSER9_SPEC>;
#[doc = "Asynchronous User Ch 9 Input Selection - Event Out 1"]
pub mod asyncuser9;
#[doc = "SYNCCH0 register accessor: an alias for `Reg<SYNCCH0_SPEC>`"]
pub type SYNCCH0 = crate::Reg<syncch0::SYNCCH0_SPEC>;
#[doc = "Synchronous Channel 0 Generator Selection"]
pub mod syncch0;
#[doc = "SYNCCH1 register accessor: an alias for `Reg<SYNCCH1_SPEC>`"]
pub type SYNCCH1 = crate::Reg<syncch1::SYNCCH1_SPEC>;
#[doc = "Synchronous Channel 1 Generator Selection"]
pub mod syncch1;
#[doc = "SYNCSTROBE register accessor: an alias for `Reg<SYNCSTROBE_SPEC>`"]
pub type SYNCSTROBE = crate::Reg<syncstrobe::SYNCSTROBE_SPEC>;
#[doc = "Synchronous Channel Strobe"]
pub mod syncstrobe;
#[doc = "SYNCUSER0 register accessor: an alias for `Reg<SYNCUSER0_SPEC>`"]
pub type SYNCUSER0 = crate::Reg<syncuser0::SYNCUSER0_SPEC>;
#[doc = "Synchronous User Ch 0 Input Selection - TCA0"]
pub mod syncuser0;
#[doc = "SYNCUSER1 register accessor: an alias for `Reg<SYNCUSER1_SPEC>`"]
pub type SYNCUSER1 = crate::Reg<syncuser1::SYNCUSER1_SPEC>;
#[doc = "Synchronous User Ch 1 Input Selection - USART0"]
pub mod syncuser1;
