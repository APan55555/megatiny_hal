#[doc = "Register `CAPTUREA` reader"]
pub struct R(crate::attiny412pac::R<CAPTUREA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CAPTUREA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CAPTUREA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CAPTUREA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Capture A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capturea](index.html) module"]
pub struct CAPTUREA_SPEC;
impl crate::attiny412pac::RegisterSpec for CAPTUREA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [capturea::R](R) reader structure"]
impl crate::attiny412pac::Readable for CAPTUREA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPTUREA to value 0"]
impl crate::attiny412pac::Resettable for CAPTUREA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
