#[doc = "Register `DEVICEID1` reader"]
pub struct R(crate::attiny412pac::R<DEVICEID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<DEVICEID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<DEVICEID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<DEVICEID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device ID Byte 1\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid1](index.html) module"]
pub struct DEVICEID1_SPEC;
impl crate::attiny412pac::RegisterSpec for DEVICEID1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceid1::R](R) reader structure"]
impl crate::attiny412pac::Readable for DEVICEID1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID1 to value 0"]
impl crate::attiny412pac::Resettable for DEVICEID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
