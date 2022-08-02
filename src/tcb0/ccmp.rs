#[doc = "Register `CCMP` reader"]
pub struct R(crate::R<CCMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMP` writer"]
pub struct W(crate::W<CCMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMP_SPEC>;
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
impl From<crate::W<CCMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMP_SPEC>) -> Self {
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
#[doc = "Compare or Capture\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmp](index.html) module"]
pub struct CCMP_SPEC;
impl crate::RegisterSpec for CCMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ccmp::R](R) reader structure"]
impl crate::Readable for CCMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmp::W](W) writer structure"]
impl crate::Writable for CCMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCMP to value 0"]
impl crate::Resettable for CCMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
