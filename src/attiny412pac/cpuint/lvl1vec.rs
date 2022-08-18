#[doc = "Register `LVL1VEC` reader"]
pub struct R(crate::attiny412pac::R<LVL1VEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<LVL1VEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<LVL1VEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<LVL1VEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVL1VEC` writer"]
pub struct W(crate::attiny412pac::W<LVL1VEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<LVL1VEC_SPEC>;
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
impl From<crate::attiny412pac::W<LVL1VEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<LVL1VEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL1VEC` reader - Interrupt Vector with High Priority"]
pub type LVL1VEC_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `LVL1VEC` writer - Interrupt Vector with High Priority"]
pub type LVL1VEC_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, LVL1VEC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    pub fn lvl1vec(&self) -> LVL1VEC_R {
        LVL1VEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt Vector with High Priority"]
    #[inline(always)]
    pub fn lvl1vec(&mut self) -> LVL1VEC_W<0> {
        LVL1VEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Interrupt Level 1 Priority Vector\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvl1vec](index.html) module"]
pub struct LVL1VEC_SPEC;
impl crate::attiny412pac::RegisterSpec for LVL1VEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvl1vec::R](R) reader structure"]
impl crate::attiny412pac::Readable for LVL1VEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvl1vec::W](W) writer structure"]
impl crate::attiny412pac::Writable for LVL1VEC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVL1VEC to value 0"]
impl crate::attiny412pac::Resettable for LVL1VEC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
