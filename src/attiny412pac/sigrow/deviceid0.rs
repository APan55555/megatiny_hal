#[doc = "Register `DEVICEID0` reader"]
pub struct R(crate::attiny412pac::R<DEVICEID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<DEVICEID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<DEVICEID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<DEVICEID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Device ID Byte 0\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [deviceid0](index.html) module"]
pub struct DEVICEID0_SPEC;
impl crate::attiny412pac::RegisterSpec for DEVICEID0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [deviceid0::R](R) reader structure"]
impl crate::attiny412pac::Readable for DEVICEID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DEVICEID0 to value 0"]
impl crate::attiny412pac::Resettable for DEVICEID0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
