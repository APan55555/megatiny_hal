#[doc = "Register `ASYNCSTROBE` reader"]
pub struct R(crate::R<ASYNCSTROBE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCSTROBE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCSTROBE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCSTROBE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCSTROBE` writer"]
pub struct W(crate::W<ASYNCSTROBE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCSTROBE_SPEC>;
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
impl From<crate::W<ASYNCSTROBE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCSTROBE_SPEC>) -> Self {
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
#[doc = "Asynchronous Channel Strobe\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncstrobe](index.html) module"]
pub struct ASYNCSTROBE_SPEC;
impl crate::RegisterSpec for ASYNCSTROBE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncstrobe::R](R) reader structure"]
impl crate::Readable for ASYNCSTROBE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncstrobe::W](W) writer structure"]
impl crate::Writable for ASYNCSTROBE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCSTROBE to value 0"]
impl crate::Resettable for ASYNCSTROBE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
