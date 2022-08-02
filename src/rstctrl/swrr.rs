#[doc = "Register `SWRR` reader"]
pub struct R(crate::R<SWRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWRR` writer"]
pub struct W(crate::W<SWRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRR_SPEC>;
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
impl From<crate::W<SWRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRE` reader - Software reset enable"]
pub type SWRE_R = crate::BitReader<bool>;
#[doc = "Field `SWRE` writer - Software reset enable"]
pub type SWRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SWRR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn swre(&self) -> SWRE_R {
        SWRE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software reset enable"]
    #[inline(always)]
    pub fn swre(&mut self) -> SWRE_W<0> {
        SWRE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrr](index.html) module"]
pub struct SWRR_SPEC;
impl crate::RegisterSpec for SWRR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [swrr::R](R) reader structure"]
impl crate::Readable for SWRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrr::W](W) writer structure"]
impl crate::Writable for SWRR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWRR to value 0"]
impl crate::Resettable for SWRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
