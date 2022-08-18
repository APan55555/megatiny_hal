#[doc = "Register `CCMP` reader"]
pub struct R(crate::attiny412pac::R<CCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMP` writer"]
pub struct W(crate::attiny412pac::W<CCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CCMP_SPEC>;
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
impl From<crate::attiny412pac::W<CCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CCMP_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare or Capture\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmp](index.html) module"]
pub struct CCMP_SPEC;
impl crate::attiny412pac::RegisterSpec for CCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ccmp::R](R) reader structure"]
impl crate::attiny412pac::Readable for CCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmp::W](W) writer structure"]
impl crate::attiny412pac::Writable for CCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMP to value 0"]
impl crate::attiny412pac::Resettable for CCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
