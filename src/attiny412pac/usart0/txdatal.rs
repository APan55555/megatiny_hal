#[doc = "Register `TXDATAL` reader"]
pub struct R(crate::attiny412pac::R<TXDATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<TXDATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<TXDATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<TXDATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATAL` writer"]
pub struct W(crate::attiny412pac::W<TXDATAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<TXDATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::attiny412pac::W<TXDATAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<TXDATAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Transmit Data Register"]
pub type DATA_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Transmit Data Register"]
pub type DATA_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, TXDATAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Data Register"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data Register"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Transmit Data Low Byte\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatal](index.html) module"]
pub struct TXDATAL_SPEC;
impl crate::attiny412pac::RegisterSpec for TXDATAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txdatal::R](R) reader structure"]
impl crate::attiny412pac::Readable for TXDATAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdatal::W](W) writer structure"]
impl crate::attiny412pac::Writable for TXDATAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATAL to value 0"]
impl crate::attiny412pac::Resettable for TXDATAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
