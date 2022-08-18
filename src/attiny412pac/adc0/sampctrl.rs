#[doc = "Register `SAMPCTRL` reader"]
pub struct R(crate::attiny412pac::R<SAMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SAMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SAMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SAMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMPCTRL` writer"]
pub struct W(crate::attiny412pac::W<SAMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SAMPCTRL_SPEC>;
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
impl From<crate::attiny412pac::W<SAMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SAMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAMPLEN` reader - Sample lenght"]
pub type SAMPLEN_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `SAMPLEN` writer - Sample lenght"]
pub type SAMPLEN_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, SAMPCTRL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Sample lenght"]
    #[inline(always)]
    pub fn samplen(&self) -> SAMPLEN_R {
        SAMPLEN_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Sample lenght"]
    #[inline(always)]
    pub fn samplen(&mut self) -> SAMPLEN_W<0> {
        SAMPLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sample Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sampctrl](index.html) module"]
pub struct SAMPCTRL_SPEC;
impl crate::attiny412pac::RegisterSpec for SAMPCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sampctrl::R](R) reader structure"]
impl crate::attiny412pac::Readable for SAMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sampctrl::W](W) writer structure"]
impl crate::attiny412pac::Writable for SAMPCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAMPCTRL to value 0"]
impl crate::attiny412pac::Resettable for SAMPCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
