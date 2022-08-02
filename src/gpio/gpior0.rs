#[doc = "Register `GPIOR0` reader"]
pub struct R(crate::R<GPIOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOR0` writer"]
pub struct W(crate::W<GPIOR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOR0_SPEC>;
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
impl From<crate::W<GPIOR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOR0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose IO Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpior0](index.html) module"]
pub struct GPIOR0_SPEC;
impl crate::RegisterSpec for GPIOR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpior0::R](R) reader structure"]
impl crate::Readable for GPIOR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpior0::W](W) writer structure"]
impl crate::Writable for GPIOR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOR0 to value 0"]
impl crate::Resettable for GPIOR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
