#[doc = "Register `SERNUM1` reader"]
pub struct R(crate::attiny412pac::R<SERNUM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SERNUM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SERNUM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SERNUM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 1\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum1](index.html) module"]
pub struct SERNUM1_SPEC;
impl crate::attiny412pac::RegisterSpec for SERNUM1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum1::R](R) reader structure"]
impl crate::attiny412pac::Readable for SERNUM1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM1 to value 0"]
impl crate::attiny412pac::Resettable for SERNUM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
