#[doc = "Register `TEMP` reader"]
pub struct R(crate::R<TEMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMP` writer"]
pub struct W(crate::W<TEMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMP_SPEC>;
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
impl From<crate::W<TEMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMP` reader - Temporary"]
pub type TEMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMP` writer - Temporary"]
pub type TEMP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TEMP_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Temporary"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Temporary"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W<0> {
        TEMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Temporary Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [temp](index.html) module"]
pub struct TEMP_SPEC;
impl crate::RegisterSpec for TEMP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [temp::R](R) reader structure"]
impl crate::Readable for TEMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [temp::W](W) writer structure"]
impl crate::Writable for TEMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TEMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
