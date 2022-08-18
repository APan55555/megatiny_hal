#[doc = "Register `SPH` reader"]
pub struct R(crate::attiny412pac::R<SPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPH` writer"]
pub struct W(crate::attiny412pac::W<SPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SPH_SPEC>;
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
impl From<crate::attiny412pac::W<SPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SPH_SPEC>) -> Self {
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
#[doc = "Stack Pointer High\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sph](index.html) module"]
pub struct SPH_SPEC;
impl crate::attiny412pac::RegisterSpec for SPH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sph::R](R) reader structure"]
impl crate::attiny412pac::Readable for SPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sph::W](W) writer structure"]
impl crate::attiny412pac::Writable for SPH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPH to value 0"]
impl crate::attiny412pac::Resettable for SPH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
