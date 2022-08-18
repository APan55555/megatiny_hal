#[doc = "Register `RES` reader"]
pub struct R(crate::attiny412pac::R<RES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<RES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<RES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<RES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ADC Accumulator Result\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [res](index.html) module"]
pub struct RES_SPEC;
impl crate::attiny412pac::RegisterSpec for RES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [res::R](R) reader structure"]
impl crate::attiny412pac::Readable for RES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RES to value 0"]
impl crate::attiny412pac::Resettable for RES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
