#[doc = "Register `INTFLAGS` reader"]
pub struct R(crate::attiny412pac::R<INTFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<INTFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<INTFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<INTFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGS` writer"]
pub struct W(crate::attiny412pac::W<INTFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<INTFLAGS_SPEC>;
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
impl From<crate::attiny412pac::W<INTFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<INTFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` reader - Overflow Interrupt Flag"]
pub type OVF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `OVF` writer - Overflow Interrupt Flag"]
pub type OVF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
#[doc = "Field `CMP` reader - Compare Match Interrupt"]
pub type CMP_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMP` writer - Compare Match Interrupt"]
pub type CMP_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTFLAGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match Interrupt"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 1 - Compare Match Interrupt"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W<1> {
        CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflags](index.html) module"]
pub struct INTFLAGS_SPEC;
impl crate::attiny412pac::RegisterSpec for INTFLAGS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intflags::R](R) reader structure"]
impl crate::attiny412pac::Readable for INTFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflags::W](W) writer structure"]
impl crate::attiny412pac::Writable for INTFLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGS to value 0"]
impl crate::attiny412pac::Resettable for INTFLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
