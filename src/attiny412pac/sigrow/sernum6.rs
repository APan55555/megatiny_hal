#[doc = "Register `SERNUM6` reader"]
pub struct R(crate::attiny412pac::R<SERNUM6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SERNUM6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SERNUM6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SERNUM6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 6\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum6](index.html) module"]
pub struct SERNUM6_SPEC;
impl crate::attiny412pac::RegisterSpec for SERNUM6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum6::R](R) reader structure"]
impl crate::attiny412pac::Readable for SERNUM6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM6 to value 0"]
impl crate::attiny412pac::Resettable for SERNUM6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
