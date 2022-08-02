#[doc = "Register `USERROW31` reader"]
pub struct R(crate::R<USERROW31_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USERROW31_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USERROW31_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USERROW31_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USERROW31` writer"]
pub struct W(crate::W<USERROW31_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USERROW31_SPEC>;
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
impl From<crate::W<USERROW31_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USERROW31_SPEC>) -> Self {
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
#[doc = "User Row Byte 31\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userrow31](index.html) module"]
pub struct USERROW31_SPEC;
impl crate::RegisterSpec for USERROW31_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [userrow31::R](R) reader structure"]
impl crate::Readable for USERROW31_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [userrow31::W](W) writer structure"]
impl crate::Writable for USERROW31_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USERROW31 to value 0"]
impl crate::Resettable for USERROW31_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
