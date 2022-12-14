#[doc = "Register `CTRLB` reader"]
pub struct R(crate::attiny412pac::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::attiny412pac::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLB_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APCWP` reader - Application code write protect"]
pub type APCWP_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `APCWP` writer - Application code write protect"]
pub type APCWP_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `BOOTLOCK` reader - Boot Lock"]
pub type BOOTLOCK_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `BOOTLOCK` writer - Boot Lock"]
pub type BOOTLOCK_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Application code write protect"]
    #[inline(always)]
    pub fn apcwp(&self) -> APCWP_R {
        APCWP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Boot Lock"]
    #[inline(always)]
    pub fn bootlock(&self) -> BOOTLOCK_R {
        BOOTLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application code write protect"]
    #[inline(always)]
    pub fn apcwp(&mut self) -> APCWP_W<0> {
        APCWP_W::new(self)
    }
    #[doc = "Bit 1 - Boot Lock"]
    #[inline(always)]
    pub fn bootlock(&mut self) -> BOOTLOCK_W<1> {
        BOOTLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::attiny412pac::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
