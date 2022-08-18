#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device ID Byte 0"]
    pub deviceid0: crate::attiny412pac::Reg<deviceid0::DEVICEID0_SPEC>,
    #[doc = "0x01 - Device ID Byte 1"]
    pub deviceid1: crate::attiny412pac::Reg<deviceid1::DEVICEID1_SPEC>,
    #[doc = "0x02 - Device ID Byte 2"]
    pub deviceid2: crate::attiny412pac::Reg<deviceid2::DEVICEID2_SPEC>,
    #[doc = "0x03 - Serial Number Byte 0"]
    pub sernum0: crate::attiny412pac::Reg<sernum0::SERNUM0_SPEC>,
    #[doc = "0x04 - Serial Number Byte 1"]
    pub sernum1: crate::attiny412pac::Reg<sernum1::SERNUM1_SPEC>,
    #[doc = "0x05 - Serial Number Byte 2"]
    pub sernum2: crate::attiny412pac::Reg<sernum2::SERNUM2_SPEC>,
    #[doc = "0x06 - Serial Number Byte 3"]
    pub sernum3: crate::attiny412pac::Reg<sernum3::SERNUM3_SPEC>,
    #[doc = "0x07 - Serial Number Byte 4"]
    pub sernum4: crate::attiny412pac::Reg<sernum4::SERNUM4_SPEC>,
    #[doc = "0x08 - Serial Number Byte 5"]
    pub sernum5: crate::attiny412pac::Reg<sernum5::SERNUM5_SPEC>,
    #[doc = "0x09 - Serial Number Byte 6"]
    pub sernum6: crate::attiny412pac::Reg<sernum6::SERNUM6_SPEC>,
    #[doc = "0x0a - Serial Number Byte 7"]
    pub sernum7: crate::attiny412pac::Reg<sernum7::SERNUM7_SPEC>,
    #[doc = "0x0b - Serial Number Byte 8"]
    pub sernum8: crate::attiny412pac::Reg<sernum8::SERNUM8_SPEC>,
    #[doc = "0x0c - Serial Number Byte 9"]
    pub sernum9: crate::attiny412pac::Reg<sernum9::SERNUM9_SPEC>,
    _reserved13: [u8; 0x13],
    #[doc = "0x20 - Temperature Sensor Calibration Byte 0"]
    pub tempsense0: crate::attiny412pac::Reg<tempsense0::TEMPSENSE0_SPEC>,
    #[doc = "0x21 - Temperature Sensor Calibration Byte 1"]
    pub tempsense1: crate::attiny412pac::Reg<tempsense1::TEMPSENSE1_SPEC>,
    #[doc = "0x22 - OSC16 error at 3V"]
    pub osc16err3v: crate::attiny412pac::Reg<osc16err3v::OSC16ERR3V_SPEC>,
    #[doc = "0x23 - OSC16 error at 5V"]
    pub osc16err5v: crate::attiny412pac::Reg<osc16err5v::OSC16ERR5V_SPEC>,
    #[doc = "0x24 - OSC20 error at 3V"]
    pub osc20err3v: crate::attiny412pac::Reg<osc20err3v::OSC20ERR3V_SPEC>,
    #[doc = "0x25 - OSC20 error at 5V"]
    pub osc20err5v: crate::attiny412pac::Reg<osc20err5v::OSC20ERR5V_SPEC>,
}
#[doc = "DEVICEID0 register accessor: an alias for `Reg<DEVICEID0_SPEC>`"]
pub type DEVICEID0 = crate::attiny412pac::Reg<deviceid0::DEVICEID0_SPEC>;
#[doc = "Device ID Byte 0"]
pub mod deviceid0;
#[doc = "DEVICEID1 register accessor: an alias for `Reg<DEVICEID1_SPEC>`"]
pub type DEVICEID1 = crate::attiny412pac::Reg<deviceid1::DEVICEID1_SPEC>;
#[doc = "Device ID Byte 1"]
pub mod deviceid1;
#[doc = "DEVICEID2 register accessor: an alias for `Reg<DEVICEID2_SPEC>`"]
pub type DEVICEID2 = crate::attiny412pac::Reg<deviceid2::DEVICEID2_SPEC>;
#[doc = "Device ID Byte 2"]
pub mod deviceid2;
#[doc = "OSC16ERR3V register accessor: an alias for `Reg<OSC16ERR3V_SPEC>`"]
pub type OSC16ERR3V = crate::attiny412pac::Reg<osc16err3v::OSC16ERR3V_SPEC>;
#[doc = "OSC16 error at 3V"]
pub mod osc16err3v;
#[doc = "OSC16ERR5V register accessor: an alias for `Reg<OSC16ERR5V_SPEC>`"]
pub type OSC16ERR5V = crate::attiny412pac::Reg<osc16err5v::OSC16ERR5V_SPEC>;
#[doc = "OSC16 error at 5V"]
pub mod osc16err5v;
#[doc = "OSC20ERR3V register accessor: an alias for `Reg<OSC20ERR3V_SPEC>`"]
pub type OSC20ERR3V = crate::attiny412pac::Reg<osc20err3v::OSC20ERR3V_SPEC>;
#[doc = "OSC20 error at 3V"]
pub mod osc20err3v;
#[doc = "OSC20ERR5V register accessor: an alias for `Reg<OSC20ERR5V_SPEC>`"]
pub type OSC20ERR5V = crate::attiny412pac::Reg<osc20err5v::OSC20ERR5V_SPEC>;
#[doc = "OSC20 error at 5V"]
pub mod osc20err5v;
#[doc = "SERNUM0 register accessor: an alias for `Reg<SERNUM0_SPEC>`"]
pub type SERNUM0 = crate::attiny412pac::Reg<sernum0::SERNUM0_SPEC>;
#[doc = "Serial Number Byte 0"]
pub mod sernum0;
#[doc = "SERNUM1 register accessor: an alias for `Reg<SERNUM1_SPEC>`"]
pub type SERNUM1 = crate::attiny412pac::Reg<sernum1::SERNUM1_SPEC>;
#[doc = "Serial Number Byte 1"]
pub mod sernum1;
#[doc = "SERNUM2 register accessor: an alias for `Reg<SERNUM2_SPEC>`"]
pub type SERNUM2 = crate::attiny412pac::Reg<sernum2::SERNUM2_SPEC>;
#[doc = "Serial Number Byte 2"]
pub mod sernum2;
#[doc = "SERNUM3 register accessor: an alias for `Reg<SERNUM3_SPEC>`"]
pub type SERNUM3 = crate::attiny412pac::Reg<sernum3::SERNUM3_SPEC>;
#[doc = "Serial Number Byte 3"]
pub mod sernum3;
#[doc = "SERNUM4 register accessor: an alias for `Reg<SERNUM4_SPEC>`"]
pub type SERNUM4 = crate::attiny412pac::Reg<sernum4::SERNUM4_SPEC>;
#[doc = "Serial Number Byte 4"]
pub mod sernum4;
#[doc = "SERNUM5 register accessor: an alias for `Reg<SERNUM5_SPEC>`"]
pub type SERNUM5 = crate::attiny412pac::Reg<sernum5::SERNUM5_SPEC>;
#[doc = "Serial Number Byte 5"]
pub mod sernum5;
#[doc = "SERNUM6 register accessor: an alias for `Reg<SERNUM6_SPEC>`"]
pub type SERNUM6 = crate::attiny412pac::Reg<sernum6::SERNUM6_SPEC>;
#[doc = "Serial Number Byte 6"]
pub mod sernum6;
#[doc = "SERNUM7 register accessor: an alias for `Reg<SERNUM7_SPEC>`"]
pub type SERNUM7 = crate::attiny412pac::Reg<sernum7::SERNUM7_SPEC>;
#[doc = "Serial Number Byte 7"]
pub mod sernum7;
#[doc = "SERNUM8 register accessor: an alias for `Reg<SERNUM8_SPEC>`"]
pub type SERNUM8 = crate::attiny412pac::Reg<sernum8::SERNUM8_SPEC>;
#[doc = "Serial Number Byte 8"]
pub mod sernum8;
#[doc = "SERNUM9 register accessor: an alias for `Reg<SERNUM9_SPEC>`"]
pub type SERNUM9 = crate::attiny412pac::Reg<sernum9::SERNUM9_SPEC>;
#[doc = "Serial Number Byte 9"]
pub mod sernum9;
#[doc = "TEMPSENSE0 register accessor: an alias for `Reg<TEMPSENSE0_SPEC>`"]
pub type TEMPSENSE0 = crate::attiny412pac::Reg<tempsense0::TEMPSENSE0_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 0"]
pub mod tempsense0;
#[doc = "TEMPSENSE1 register accessor: an alias for `Reg<TEMPSENSE1_SPEC>`"]
pub type TEMPSENSE1 = crate::attiny412pac::Reg<tempsense1::TEMPSENSE1_SPEC>;
#[doc = "Temperature Sensor Calibration Byte 1"]
pub mod tempsense1;
