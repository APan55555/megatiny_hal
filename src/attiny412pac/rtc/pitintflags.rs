#[doc = "Register `PITINTFLAGS` reader"]
pub struct R(crate::attiny412pac::R<PITINTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<PITINTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<PITINTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<PITINTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PITINTFLAGS` writer"]
pub struct W(crate::attiny412pac::W<PITINTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<PITINTFLAGS_SPEC>;
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
impl From<crate::attiny412pac::W<PITINTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<PITINTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PI` reader - Periodic Interrupt"]
pub type PI_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PI` writer - Periodic Interrupt"]
pub type PI_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, PITINTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Periodic Interrupt"]
    #[inline(always)]
    pub fn pi(&mut self) -> PI_W<0> {
        PI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Interrupt Flags\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitintflags](index.html) module"]
pub struct PITINTFLAGS_SPEC;
impl crate::attiny412pac::RegisterSpec for PITINTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitintflags::R](R) reader structure"]
impl crate::attiny412pac::Readable for PITINTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pitintflags::W](W) writer structure"]
impl crate::attiny412pac::Writable for PITINTFLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PITINTFLAGS to value 0"]
impl crate::attiny412pac::Resettable for PITINTFLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
