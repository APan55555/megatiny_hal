#[doc = "Register `USERROW16` reader"]
pub struct R(crate::attiny412pac::R<USERROW16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<USERROW16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<USERROW16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<USERROW16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERROW16` writer"]
pub struct W(crate::attiny412pac::W<USERROW16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<USERROW16_SPEC>;
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
impl From<crate::attiny412pac::W<USERROW16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<USERROW16_SPEC>) -> Self {
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
#[doc = "User Row Byte 16\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userrow16](index.html) module"]
pub struct USERROW16_SPEC;
impl crate::attiny412pac::RegisterSpec for USERROW16_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [userrow16::R](R) reader structure"]
impl crate::attiny412pac::Readable for USERROW16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userrow16::W](W) writer structure"]
impl crate::attiny412pac::Writable for USERROW16_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USERROW16 to value 0"]
impl crate::attiny412pac::Resettable for USERROW16_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
