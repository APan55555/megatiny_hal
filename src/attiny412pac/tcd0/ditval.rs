#[doc = "Register `DITVAL` reader"]
pub struct R(crate::attiny412pac::R<DITVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<DITVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<DITVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<DITVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DITVAL` writer"]
pub struct W(crate::attiny412pac::W<DITVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<DITVAL_SPEC>;
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
impl From<crate::attiny412pac::W<DITVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<DITVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DITHER` reader - Dither value"]
pub type DITHER_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `DITHER` writer - Dither value"]
pub type DITHER_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, DITVAL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Dither value"]
    #[inline(always)]
    pub fn dither(&self) -> DITHER_R {
        DITHER_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Dither value"]
    #[inline(always)]
    pub fn dither(&mut self) -> DITHER_W<0> {
        DITHER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dither value\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ditval](index.html) module"]
pub struct DITVAL_SPEC;
impl crate::attiny412pac::RegisterSpec for DITVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ditval::R](R) reader structure"]
impl crate::attiny412pac::Readable for DITVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ditval::W](W) writer structure"]
impl crate::attiny412pac::Writable for DITVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DITVAL to value 0"]
impl crate::attiny412pac::Resettable for DITVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
