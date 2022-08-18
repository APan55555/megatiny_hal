#[doc = "Register `DLYVAL` reader"]
pub struct R(crate::attiny412pac::R<DLYVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<DLYVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<DLYVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<DLYVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLYVAL` writer"]
pub struct W(crate::attiny412pac::W<DLYVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<DLYVAL_SPEC>;
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
impl From<crate::attiny412pac::W<DLYVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<DLYVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYVAL` reader - Delay value"]
pub type DLYVAL_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `DLYVAL` writer - Delay value"]
pub type DLYVAL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, DLYVAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Delay value"]
    #[inline(always)]
    pub fn dlyval(&self) -> DLYVAL_R {
        DLYVAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delay value"]
    #[inline(always)]
    pub fn dlyval(&mut self) -> DLYVAL_W<0> {
        DLYVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Delay value\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dlyval](index.html) module"]
pub struct DLYVAL_SPEC;
impl crate::attiny412pac::RegisterSpec for DLYVAL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dlyval::R](R) reader structure"]
impl crate::attiny412pac::Readable for DLYVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dlyval::W](W) writer structure"]
impl crate::attiny412pac::Writable for DLYVAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DLYVAL to value 0"]
impl crate::attiny412pac::Resettable for DLYVAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
