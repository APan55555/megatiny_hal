#[doc = "Register `CTRLD` reader"]
pub struct R(crate::attiny412pac::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::attiny412pac::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLD_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPAVAL` reader - Compare A value"]
pub type CMPAVAL_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `CMPAVAL` writer - Compare A value"]
pub type CMPAVAL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, CTRLD_SPEC, u8, u8, 4, O>;
#[doc = "Field `CMPBVAL` reader - Compare B value"]
pub type CMPBVAL_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `CMPBVAL` writer - Compare B value"]
pub type CMPBVAL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, CTRLD_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Compare A value"]
    #[inline(always)]
    pub fn cmpaval(&self) -> CMPAVAL_R {
        CMPAVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Compare B value"]
    #[inline(always)]
    pub fn cmpbval(&self) -> CMPBVAL_R {
        CMPBVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compare A value"]
    #[inline(always)]
    pub fn cmpaval(&mut self) -> CMPAVAL_W<0> {
        CMPAVAL_W::new(self)
    }
    #[doc = "Bits 4:7 - Compare B value"]
    #[inline(always)]
    pub fn cmpbval(&mut self) -> CMPBVAL_W<4> {
        CMPBVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control D\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::attiny412pac::Resettable for CTRLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
