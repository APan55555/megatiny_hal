#[doc = "Register `RSTFR` reader"]
pub struct R(crate::R<RSTFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTFR` writer"]
pub struct W(crate::W<RSTFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTFR_SPEC>;
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
impl From<crate::W<RSTFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORF` reader - Power on Reset flag"]
pub type PORF_R = crate::BitReader<bool>;
#[doc = "Field `PORF` writer - Power on Reset flag"]
pub type PORF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
#[doc = "Field `BORF` reader - Brown out detector Reset flag"]
pub type BORF_R = crate::BitReader<bool>;
#[doc = "Field `BORF` writer - Brown out detector Reset flag"]
pub type BORF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
#[doc = "Field `EXTRF` reader - External Reset flag"]
pub type EXTRF_R = crate::BitReader<bool>;
#[doc = "Field `EXTRF` writer - External Reset flag"]
pub type EXTRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
#[doc = "Field `WDRF` reader - Watch dog Reset flag"]
pub type WDRF_R = crate::BitReader<bool>;
#[doc = "Field `WDRF` writer - Watch dog Reset flag"]
pub type WDRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
#[doc = "Field `SWRF` reader - Software Reset flag"]
pub type SWRF_R = crate::BitReader<bool>;
#[doc = "Field `SWRF` writer - Software Reset flag"]
pub type SWRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
#[doc = "Field `UPDIRF` reader - UPDI Reset flag"]
pub type UPDIRF_R = crate::BitReader<bool>;
#[doc = "Field `UPDIRF` writer - UPDI Reset flag"]
pub type UPDIRF_W<'a, const O: u8> = crate::BitWriter<'a, u8, RSTFR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Power on Reset flag"]
    #[inline(always)]
    pub fn porf(&self) -> PORF_R {
        PORF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Brown out detector Reset flag"]
    #[inline(always)]
    pub fn borf(&self) -> BORF_R {
        BORF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reset flag"]
    #[inline(always)]
    pub fn extrf(&self) -> EXTRF_R {
        EXTRF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watch dog Reset flag"]
    #[inline(always)]
    pub fn wdrf(&self) -> WDRF_R {
        WDRF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Reset flag"]
    #[inline(always)]
    pub fn swrf(&self) -> SWRF_R {
        SWRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UPDI Reset flag"]
    #[inline(always)]
    pub fn updirf(&self) -> UPDIRF_R {
        UPDIRF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on Reset flag"]
    #[inline(always)]
    pub fn porf(&mut self) -> PORF_W<0> {
        PORF_W::new(self)
    }
    #[doc = "Bit 1 - Brown out detector Reset flag"]
    #[inline(always)]
    pub fn borf(&mut self) -> BORF_W<1> {
        BORF_W::new(self)
    }
    #[doc = "Bit 2 - External Reset flag"]
    #[inline(always)]
    pub fn extrf(&mut self) -> EXTRF_W<2> {
        EXTRF_W::new(self)
    }
    #[doc = "Bit 3 - Watch dog Reset flag"]
    #[inline(always)]
    pub fn wdrf(&mut self) -> WDRF_W<3> {
        WDRF_W::new(self)
    }
    #[doc = "Bit 4 - Software Reset flag"]
    #[inline(always)]
    pub fn swrf(&mut self) -> SWRF_W<4> {
        SWRF_W::new(self)
    }
    #[doc = "Bit 5 - UPDI Reset flag"]
    #[inline(always)]
    pub fn updirf(&mut self) -> UPDIRF_W<5> {
        UPDIRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstfr](index.html) module"]
pub struct RSTFR_SPEC;
impl crate::RegisterSpec for RSTFR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rstfr::R](R) reader structure"]
impl crate::Readable for RSTFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstfr::W](W) writer structure"]
impl crate::Writable for RSTFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTFR to value 0"]
impl crate::Resettable for RSTFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
