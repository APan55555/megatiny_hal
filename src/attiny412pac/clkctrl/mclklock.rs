#[doc = "Register `MCLKLOCK` reader"]
pub struct R(crate::attiny412pac::R<MCLKLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<MCLKLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<MCLKLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<MCLKLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCLKLOCK` writer"]
pub struct W(crate::attiny412pac::W<MCLKLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<MCLKLOCK_SPEC>;
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
impl From<crate::attiny412pac::W<MCLKLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<MCLKLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKEN` reader - lock ebable"]
pub type LOCKEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `LOCKEN` writer - lock ebable"]
pub type LOCKEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCLKLOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - lock ebable"]
    #[inline(always)]
    pub fn locken(&self) -> LOCKEN_R {
        LOCKEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - lock ebable"]
    #[inline(always)]
    pub fn locken(&mut self) -> LOCKEN_W<0> {
        LOCKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Lock\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclklock](index.html) module"]
pub struct MCLKLOCK_SPEC;
impl crate::attiny412pac::RegisterSpec for MCLKLOCK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mclklock::R](R) reader structure"]
impl crate::attiny412pac::Readable for MCLKLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclklock::W](W) writer structure"]
impl crate::attiny412pac::Writable for MCLKLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCLKLOCK to value 0"]
impl crate::attiny412pac::Resettable for MCLKLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
