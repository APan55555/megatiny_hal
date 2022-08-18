#[doc = "Register `TXPLCTRL` reader"]
pub struct R(crate::attiny412pac::R<TXPLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<TXPLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<TXPLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<TXPLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXPLCTRL` writer"]
pub struct W(crate::attiny412pac::W<TXPLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<TXPLCTRL_SPEC>;
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
impl From<crate::attiny412pac::W<TXPLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<TXPLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPL` reader - Transmit pulse length"]
pub type TXPL_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `TXPL` writer - Transmit pulse length"]
pub type TXPL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, TXPLCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn txpl(&self) -> TXPL_R {
        TXPL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn txpl(&mut self) -> TXPL_W<0> {
        TXPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "IRCOM Transmitter Pulse Length Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txplctrl](index.html) module"]
pub struct TXPLCTRL_SPEC;
impl crate::attiny412pac::RegisterSpec for TXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txplctrl::R](R) reader structure"]
impl crate::attiny412pac::Readable for TXPLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txplctrl::W](W) writer structure"]
impl crate::attiny412pac::Writable for TXPLCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXPLCTRL to value 0"]
impl crate::attiny412pac::Resettable for TXPLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
