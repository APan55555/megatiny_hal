#[doc = "Register `RXPLCTRL` reader"]
pub struct R(crate::attiny412pac::R<RXPLCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<RXPLCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<RXPLCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<RXPLCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXPLCTRL` writer"]
pub struct W(crate::attiny412pac::W<RXPLCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<RXPLCTRL_SPEC>;
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
impl From<crate::attiny412pac::W<RXPLCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<RXPLCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPL` reader - Receiver Pulse Lenght"]
pub type RXPL_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `RXPL` writer - Receiver Pulse Lenght"]
pub type RXPL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, RXPLCTRL_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    pub fn rxpl(&self) -> RXPL_R {
        RXPL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Receiver Pulse Lenght"]
    #[inline(always)]
    pub fn rxpl(&mut self) -> RXPL_W<0> {
        RXPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IRCOM Receiver Pulse Length Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxplctrl](index.html) module"]
pub struct RXPLCTRL_SPEC;
impl crate::attiny412pac::RegisterSpec for RXPLCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxplctrl::R](R) reader structure"]
impl crate::attiny412pac::Readable for RXPLCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxplctrl::W](W) writer structure"]
impl crate::attiny412pac::Writable for RXPLCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXPLCTRL to value 0"]
impl crate::attiny412pac::Resettable for RXPLCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
