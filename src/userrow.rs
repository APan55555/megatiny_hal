#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - User Row Byte 0"]
    pub userrow0: crate::Reg<userrow0::USERROW0_SPEC>,
    #[doc = "0x01 - User Row Byte 1"]
    pub userrow1: crate::Reg<userrow1::USERROW1_SPEC>,
    #[doc = "0x02 - User Row Byte 2"]
    pub userrow2: crate::Reg<userrow2::USERROW2_SPEC>,
    #[doc = "0x03 - User Row Byte 3"]
    pub userrow3: crate::Reg<userrow3::USERROW3_SPEC>,
    #[doc = "0x04 - User Row Byte 4"]
    pub userrow4: crate::Reg<userrow4::USERROW4_SPEC>,
    #[doc = "0x05 - User Row Byte 5"]
    pub userrow5: crate::Reg<userrow5::USERROW5_SPEC>,
    #[doc = "0x06 - User Row Byte 6"]
    pub userrow6: crate::Reg<userrow6::USERROW6_SPEC>,
    #[doc = "0x07 - User Row Byte 7"]
    pub userrow7: crate::Reg<userrow7::USERROW7_SPEC>,
    #[doc = "0x08 - User Row Byte 8"]
    pub userrow8: crate::Reg<userrow8::USERROW8_SPEC>,
    #[doc = "0x09 - User Row Byte 9"]
    pub userrow9: crate::Reg<userrow9::USERROW9_SPEC>,
    #[doc = "0x0a - User Row Byte 10"]
    pub userrow10: crate::Reg<userrow10::USERROW10_SPEC>,
    #[doc = "0x0b - User Row Byte 11"]
    pub userrow11: crate::Reg<userrow11::USERROW11_SPEC>,
    #[doc = "0x0c - User Row Byte 12"]
    pub userrow12: crate::Reg<userrow12::USERROW12_SPEC>,
    #[doc = "0x0d - User Row Byte 13"]
    pub userrow13: crate::Reg<userrow13::USERROW13_SPEC>,
    #[doc = "0x0e - User Row Byte 14"]
    pub userrow14: crate::Reg<userrow14::USERROW14_SPEC>,
    #[doc = "0x0f - User Row Byte 15"]
    pub userrow15: crate::Reg<userrow15::USERROW15_SPEC>,
    #[doc = "0x10 - User Row Byte 16"]
    pub userrow16: crate::Reg<userrow16::USERROW16_SPEC>,
    #[doc = "0x11 - User Row Byte 17"]
    pub userrow17: crate::Reg<userrow17::USERROW17_SPEC>,
    #[doc = "0x12 - User Row Byte 18"]
    pub userrow18: crate::Reg<userrow18::USERROW18_SPEC>,
    #[doc = "0x13 - User Row Byte 19"]
    pub userrow19: crate::Reg<userrow19::USERROW19_SPEC>,
    #[doc = "0x14 - User Row Byte 20"]
    pub userrow20: crate::Reg<userrow20::USERROW20_SPEC>,
    #[doc = "0x15 - User Row Byte 21"]
    pub userrow21: crate::Reg<userrow21::USERROW21_SPEC>,
    #[doc = "0x16 - User Row Byte 22"]
    pub userrow22: crate::Reg<userrow22::USERROW22_SPEC>,
    #[doc = "0x17 - User Row Byte 23"]
    pub userrow23: crate::Reg<userrow23::USERROW23_SPEC>,
    #[doc = "0x18 - User Row Byte 24"]
    pub userrow24: crate::Reg<userrow24::USERROW24_SPEC>,
    #[doc = "0x19 - User Row Byte 25"]
    pub userrow25: crate::Reg<userrow25::USERROW25_SPEC>,
    #[doc = "0x1a - User Row Byte 26"]
    pub userrow26: crate::Reg<userrow26::USERROW26_SPEC>,
    #[doc = "0x1b - User Row Byte 27"]
    pub userrow27: crate::Reg<userrow27::USERROW27_SPEC>,
    #[doc = "0x1c - User Row Byte 28"]
    pub userrow28: crate::Reg<userrow28::USERROW28_SPEC>,
    #[doc = "0x1d - User Row Byte 29"]
    pub userrow29: crate::Reg<userrow29::USERROW29_SPEC>,
    #[doc = "0x1e - User Row Byte 30"]
    pub userrow30: crate::Reg<userrow30::USERROW30_SPEC>,
    #[doc = "0x1f - User Row Byte 31"]
    pub userrow31: crate::Reg<userrow31::USERROW31_SPEC>,
}
#[doc = "USERROW0 register accessor: an alias for `Reg<USERROW0_SPEC>`"]
pub type USERROW0 = crate::Reg<userrow0::USERROW0_SPEC>;
#[doc = "User Row Byte 0"]
pub mod userrow0;
#[doc = "USERROW1 register accessor: an alias for `Reg<USERROW1_SPEC>`"]
pub type USERROW1 = crate::Reg<userrow1::USERROW1_SPEC>;
#[doc = "User Row Byte 1"]
pub mod userrow1;
#[doc = "USERROW10 register accessor: an alias for `Reg<USERROW10_SPEC>`"]
pub type USERROW10 = crate::Reg<userrow10::USERROW10_SPEC>;
#[doc = "User Row Byte 10"]
pub mod userrow10;
#[doc = "USERROW11 register accessor: an alias for `Reg<USERROW11_SPEC>`"]
pub type USERROW11 = crate::Reg<userrow11::USERROW11_SPEC>;
#[doc = "User Row Byte 11"]
pub mod userrow11;
#[doc = "USERROW12 register accessor: an alias for `Reg<USERROW12_SPEC>`"]
pub type USERROW12 = crate::Reg<userrow12::USERROW12_SPEC>;
#[doc = "User Row Byte 12"]
pub mod userrow12;
#[doc = "USERROW13 register accessor: an alias for `Reg<USERROW13_SPEC>`"]
pub type USERROW13 = crate::Reg<userrow13::USERROW13_SPEC>;
#[doc = "User Row Byte 13"]
pub mod userrow13;
#[doc = "USERROW14 register accessor: an alias for `Reg<USERROW14_SPEC>`"]
pub type USERROW14 = crate::Reg<userrow14::USERROW14_SPEC>;
#[doc = "User Row Byte 14"]
pub mod userrow14;
#[doc = "USERROW15 register accessor: an alias for `Reg<USERROW15_SPEC>`"]
pub type USERROW15 = crate::Reg<userrow15::USERROW15_SPEC>;
#[doc = "User Row Byte 15"]
pub mod userrow15;
#[doc = "USERROW16 register accessor: an alias for `Reg<USERROW16_SPEC>`"]
pub type USERROW16 = crate::Reg<userrow16::USERROW16_SPEC>;
#[doc = "User Row Byte 16"]
pub mod userrow16;
#[doc = "USERROW17 register accessor: an alias for `Reg<USERROW17_SPEC>`"]
pub type USERROW17 = crate::Reg<userrow17::USERROW17_SPEC>;
#[doc = "User Row Byte 17"]
pub mod userrow17;
#[doc = "USERROW18 register accessor: an alias for `Reg<USERROW18_SPEC>`"]
pub type USERROW18 = crate::Reg<userrow18::USERROW18_SPEC>;
#[doc = "User Row Byte 18"]
pub mod userrow18;
#[doc = "USERROW19 register accessor: an alias for `Reg<USERROW19_SPEC>`"]
pub type USERROW19 = crate::Reg<userrow19::USERROW19_SPEC>;
#[doc = "User Row Byte 19"]
pub mod userrow19;
#[doc = "USERROW2 register accessor: an alias for `Reg<USERROW2_SPEC>`"]
pub type USERROW2 = crate::Reg<userrow2::USERROW2_SPEC>;
#[doc = "User Row Byte 2"]
pub mod userrow2;
#[doc = "USERROW20 register accessor: an alias for `Reg<USERROW20_SPEC>`"]
pub type USERROW20 = crate::Reg<userrow20::USERROW20_SPEC>;
#[doc = "User Row Byte 20"]
pub mod userrow20;
#[doc = "USERROW21 register accessor: an alias for `Reg<USERROW21_SPEC>`"]
pub type USERROW21 = crate::Reg<userrow21::USERROW21_SPEC>;
#[doc = "User Row Byte 21"]
pub mod userrow21;
#[doc = "USERROW22 register accessor: an alias for `Reg<USERROW22_SPEC>`"]
pub type USERROW22 = crate::Reg<userrow22::USERROW22_SPEC>;
#[doc = "User Row Byte 22"]
pub mod userrow22;
#[doc = "USERROW23 register accessor: an alias for `Reg<USERROW23_SPEC>`"]
pub type USERROW23 = crate::Reg<userrow23::USERROW23_SPEC>;
#[doc = "User Row Byte 23"]
pub mod userrow23;
#[doc = "USERROW24 register accessor: an alias for `Reg<USERROW24_SPEC>`"]
pub type USERROW24 = crate::Reg<userrow24::USERROW24_SPEC>;
#[doc = "User Row Byte 24"]
pub mod userrow24;
#[doc = "USERROW25 register accessor: an alias for `Reg<USERROW25_SPEC>`"]
pub type USERROW25 = crate::Reg<userrow25::USERROW25_SPEC>;
#[doc = "User Row Byte 25"]
pub mod userrow25;
#[doc = "USERROW26 register accessor: an alias for `Reg<USERROW26_SPEC>`"]
pub type USERROW26 = crate::Reg<userrow26::USERROW26_SPEC>;
#[doc = "User Row Byte 26"]
pub mod userrow26;
#[doc = "USERROW27 register accessor: an alias for `Reg<USERROW27_SPEC>`"]
pub type USERROW27 = crate::Reg<userrow27::USERROW27_SPEC>;
#[doc = "User Row Byte 27"]
pub mod userrow27;
#[doc = "USERROW28 register accessor: an alias for `Reg<USERROW28_SPEC>`"]
pub type USERROW28 = crate::Reg<userrow28::USERROW28_SPEC>;
#[doc = "User Row Byte 28"]
pub mod userrow28;
#[doc = "USERROW29 register accessor: an alias for `Reg<USERROW29_SPEC>`"]
pub type USERROW29 = crate::Reg<userrow29::USERROW29_SPEC>;
#[doc = "User Row Byte 29"]
pub mod userrow29;
#[doc = "USERROW3 register accessor: an alias for `Reg<USERROW3_SPEC>`"]
pub type USERROW3 = crate::Reg<userrow3::USERROW3_SPEC>;
#[doc = "User Row Byte 3"]
pub mod userrow3;
#[doc = "USERROW30 register accessor: an alias for `Reg<USERROW30_SPEC>`"]
pub type USERROW30 = crate::Reg<userrow30::USERROW30_SPEC>;
#[doc = "User Row Byte 30"]
pub mod userrow30;
#[doc = "USERROW31 register accessor: an alias for `Reg<USERROW31_SPEC>`"]
pub type USERROW31 = crate::Reg<userrow31::USERROW31_SPEC>;
#[doc = "User Row Byte 31"]
pub mod userrow31;
#[doc = "USERROW4 register accessor: an alias for `Reg<USERROW4_SPEC>`"]
pub type USERROW4 = crate::Reg<userrow4::USERROW4_SPEC>;
#[doc = "User Row Byte 4"]
pub mod userrow4;
#[doc = "USERROW5 register accessor: an alias for `Reg<USERROW5_SPEC>`"]
pub type USERROW5 = crate::Reg<userrow5::USERROW5_SPEC>;
#[doc = "User Row Byte 5"]
pub mod userrow5;
#[doc = "USERROW6 register accessor: an alias for `Reg<USERROW6_SPEC>`"]
pub type USERROW6 = crate::Reg<userrow6::USERROW6_SPEC>;
#[doc = "User Row Byte 6"]
pub mod userrow6;
#[doc = "USERROW7 register accessor: an alias for `Reg<USERROW7_SPEC>`"]
pub type USERROW7 = crate::Reg<userrow7::USERROW7_SPEC>;
#[doc = "User Row Byte 7"]
pub mod userrow7;
#[doc = "USERROW8 register accessor: an alias for `Reg<USERROW8_SPEC>`"]
pub type USERROW8 = crate::Reg<userrow8::USERROW8_SPEC>;
#[doc = "User Row Byte 8"]
pub mod userrow8;
#[doc = "USERROW9 register accessor: an alias for `Reg<USERROW9_SPEC>`"]
pub type USERROW9 = crate::Reg<userrow9::USERROW9_SPEC>;
#[doc = "User Row Byte 9"]
pub mod userrow9;
