#[doc = "Register `GPIOR3` reader"]
pub struct R(crate::attiny412pac::R<GPIOR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<GPIOR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<GPIOR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<GPIOR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOR3` writer"]
pub struct W(crate::attiny412pac::W<GPIOR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<GPIOR3_SPEC>;
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
impl From<crate::attiny412pac::W<GPIOR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<GPIOR3_SPEC>) -> Self {
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
#[doc = "General Purpose IO Register 3\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpior3](index.html) module"]
pub struct GPIOR3_SPEC;
impl crate::attiny412pac::RegisterSpec for GPIOR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpior3::R](R) reader structure"]
impl crate::attiny412pac::Readable for GPIOR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpior3::W](W) writer structure"]
impl crate::attiny412pac::Writable for GPIOR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOR3 to value 0"]
impl crate::attiny412pac::Resettable for GPIOR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
