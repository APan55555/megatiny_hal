#[doc = "Register `RXDATAL` reader"]
pub struct R(crate::attiny412pac::R<RXDATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<RXDATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<RXDATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<RXDATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - RX Data"]
pub type DATA_R = crate::attiny412pac::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - RX Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "Receive Data Low Byte\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatal](index.html) module"]
pub struct RXDATAL_SPEC;
impl crate::attiny412pac::RegisterSpec for RXDATAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxdatal::R](R) reader structure"]
impl crate::attiny412pac::Readable for RXDATAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAL to value 0"]
impl crate::attiny412pac::Resettable for RXDATAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
