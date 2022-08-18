#[doc = "Register `SCTRLA` reader"]
pub struct R(crate::attiny412pac::R<SCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTRLA` writer"]
pub struct W(crate::attiny412pac::W<SCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SCTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<SCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable TWI Slave"]
pub type ENABLE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable TWI Slave"]
pub type ENABLE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
#[doc = "Field `PMEN` reader - Promiscuous Mode Enable"]
pub type PMEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PMEN` writer - Promiscuous Mode Enable"]
pub type PMEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
#[doc = "Field `PIEN` reader - Stop Interrupt Enable"]
pub type PIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PIEN` writer - Stop Interrupt Enable"]
pub type PIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
#[doc = "Field `APIEN` reader - Address/Stop Interrupt Enable"]
pub type APIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `APIEN` writer - Address/Stop Interrupt Enable"]
pub type APIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
#[doc = "Field `DIEN` reader - Data Interrupt Enable"]
pub type DIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `DIEN` writer - Data Interrupt Enable"]
pub type DIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable TWI Slave"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Promiscuous Mode Enable"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn pien(&self) -> PIEN_R {
        PIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Enable"]
    #[inline(always)]
    pub fn apien(&self) -> APIEN_R {
        APIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TWI Slave"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bit 2 - Promiscuous Mode Enable"]
    #[inline(always)]
    pub fn pmen(&mut self) -> PMEN_W<2> {
        PMEN_W::new(self)
    }
    #[doc = "Bit 5 - Stop Interrupt Enable"]
    #[inline(always)]
    pub fn pien(&mut self) -> PIEN_W<5> {
        PIEN_W::new(self)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Enable"]
    #[inline(always)]
    pub fn apien(&mut self) -> APIEN_W<6> {
        APIEN_W::new(self)
    }
    #[doc = "Bit 7 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W<7> {
        DIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctrla](index.html) module"]
pub struct SCTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for SCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for SCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for SCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCTRLA to value 0"]
impl crate::attiny412pac::Resettable for SCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
