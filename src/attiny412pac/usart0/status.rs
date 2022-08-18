#[doc = "Register `STATUS` reader"]
pub struct R(crate::attiny412pac::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::attiny412pac::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<STATUS_SPEC>;
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
impl From<crate::attiny412pac::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WFB` reader - Wait For Break"]
pub type WFB_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `WFB` writer - Wait For Break"]
pub type WFB_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `BDF` reader - Break Detected Flag"]
pub type BDF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `BDF` writer - Break Detected Flag"]
pub type BDF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `ISFIF` reader - Inconsistent Sync Field Interrupt Flag"]
pub type ISFIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ISFIF` writer - Inconsistent Sync Field Interrupt Flag"]
pub type ISFIF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `RXSIF` reader - Receive Start Interrupt"]
pub type RXSIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `DREIF` reader - Data Register Empty Flag"]
pub type DREIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `TXCIF` reader - Transmit Interrupt Flag"]
pub type TXCIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `TXCIF` writer - Transmit Interrupt Flag"]
pub type TXCIF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::attiny412pac::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Wait For Break"]
    #[inline(always)]
    pub fn wfb(&self) -> WFB_R {
        WFB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Break Detected Flag"]
    #[inline(always)]
    pub fn bdf(&self) -> BDF_R {
        BDF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Inconsistent Sync Field Interrupt Flag"]
    #[inline(always)]
    pub fn isfif(&self) -> ISFIF_R {
        ISFIF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Start Interrupt"]
    #[inline(always)]
    pub fn rxsif(&self) -> RXSIF_R {
        RXSIF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Flag"]
    #[inline(always)]
    pub fn dreif(&self) -> DREIF_R {
        DREIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn txcif(&self) -> TXCIF_R {
        TXCIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wait For Break"]
    #[inline(always)]
    pub fn wfb(&mut self) -> WFB_W<0> {
        WFB_W::new(self)
    }
    #[doc = "Bit 1 - Break Detected Flag"]
    #[inline(always)]
    pub fn bdf(&mut self) -> BDF_W<1> {
        BDF_W::new(self)
    }
    #[doc = "Bit 3 - Inconsistent Sync Field Interrupt Flag"]
    #[inline(always)]
    pub fn isfif(&mut self) -> ISFIF_W<3> {
        ISFIF_W::new(self)
    }
    #[doc = "Bit 6 - Transmit Interrupt Flag"]
    #[inline(always)]
    pub fn txcif(&mut self) -> TXCIF_W<6> {
        TXCIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::attiny412pac::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::attiny412pac::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::attiny412pac::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::attiny412pac::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
