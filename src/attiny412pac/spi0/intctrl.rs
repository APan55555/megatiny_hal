#[doc = "Register `INTCTRL` reader"]
pub struct R(crate::attiny412pac::R<INTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<INTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<INTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<INTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCTRL` writer"]
pub struct W(crate::attiny412pac::W<INTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<INTCTRL_SPEC>;
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
impl From<crate::attiny412pac::W<INTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<INTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `SSIE` reader - Slave Select Trigger Interrupt Enable"]
pub type SSIE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `SSIE` writer - Slave Select Trigger Interrupt Enable"]
pub type SSIE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `DREIE` reader - Data Register Empty Interrupt Enable"]
pub type DREIE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `DREIE` writer - Data Register Empty Interrupt Enable"]
pub type DREIE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `TXCIE` reader - Transfer Complete Interrupt Enable"]
pub type TXCIE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `TXCIE` writer - Transfer Complete Interrupt Enable"]
pub type TXCIE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Field `RXCIE` reader - Receive Complete Interrupt Enable"]
pub type RXCIE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RXCIE` writer - Receive Complete Interrupt Enable"]
pub type RXCIE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Select Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SSIE_R {
        SSIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&self) -> DREIE_R {
        DREIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&self) -> RXCIE_R {
        RXCIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W<0> {
        IE_W::new(self)
    }
    #[doc = "Bit 4 - Slave Select Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&mut self) -> SSIE_W<4> {
        SSIE_W::new(self)
    }
    #[doc = "Bit 5 - Data Register Empty Interrupt Enable"]
    #[inline(always)]
    pub fn dreie(&mut self) -> DREIE_W<5> {
        DREIE_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn txcie(&mut self) -> TXCIE_W<6> {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Enable"]
    #[inline(always)]
    pub fn rxcie(&mut self) -> RXCIE_W<7> {
        RXCIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
pub struct INTCTRL_SPEC;
impl crate::attiny412pac::RegisterSpec for INTCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [intctrl::R](R) reader structure"]
impl crate::attiny412pac::Readable for INTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intctrl::W](W) writer structure"]
impl crate::attiny412pac::Writable for INTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCTRL to value 0"]
impl crate::attiny412pac::Resettable for INTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
