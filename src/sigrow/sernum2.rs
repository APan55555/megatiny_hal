#[doc = "Register `SERNUM2` reader"]
pub struct R(crate::R<SERNUM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SERNUM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SERNUM2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SERNUM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Serial Number Byte 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sernum2](index.html) module"]
pub struct SERNUM2_SPEC;
impl crate::RegisterSpec for SERNUM2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sernum2::R](R) reader structure"]
impl crate::Readable for SERNUM2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SERNUM2 to value 0"]
impl crate::Resettable for SERNUM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
