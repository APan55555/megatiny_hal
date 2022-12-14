#[doc = "Register `SADDRMASK` reader"]
pub struct R(crate::attiny412pac::R<SADDRMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SADDRMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SADDRMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SADDRMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDRMASK` writer"]
pub struct W(crate::attiny412pac::W<SADDRMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SADDRMASK_SPEC>;
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
impl From<crate::attiny412pac::W<SADDRMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SADDRMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDREN` reader - Address Enable"]
pub type ADDREN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ADDREN` writer - Address Enable"]
pub type ADDREN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SADDRMASK_SPEC, bool, O>;
#[doc = "Field `ADDRMASK` reader - Address Mask"]
pub type ADDRMASK_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `ADDRMASK` writer - Address Mask"]
pub type ADDRMASK_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, SADDRMASK_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Address Enable"]
    #[inline(always)]
    pub fn addren(&self) -> ADDREN_R {
        ADDREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&self) -> ADDRMASK_R {
        ADDRMASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Address Enable"]
    #[inline(always)]
    pub fn addren(&mut self) -> ADDREN_W<0> {
        ADDREN_W::new(self)
    }
    #[doc = "Bits 1:7 - Address Mask"]
    #[inline(always)]
    pub fn addrmask(&mut self) -> ADDRMASK_W<1> {
        ADDRMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Mask\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddrmask](index.html) module"]
pub struct SADDRMASK_SPEC;
impl crate::attiny412pac::RegisterSpec for SADDRMASK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [saddrmask::R](R) reader structure"]
impl crate::attiny412pac::Readable for SADDRMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddrmask::W](W) writer structure"]
impl crate::attiny412pac::Writable for SADDRMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::attiny412pac::Resettable for SADDRMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
