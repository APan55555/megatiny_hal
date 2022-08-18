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
#[doc = "Field `VLMIE` reader - voltage level monitor interrrupt enable"]
pub type VLMIE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `VLMIE` writer - voltage level monitor interrrupt enable"]
pub type VLMIE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, INTCTRL_SPEC, bool, O>;
#[doc = "Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VLMCFG_A {
    #[doc = "0: Interrupt when supply goes below VLM level"]
    BELOW = 0,
    #[doc = "1: Interrupt when supply goes above VLM level"]
    ABOVE = 1,
    #[doc = "2: Interrupt when supply crosses VLM level"]
    CROSS = 2,
}
impl From<VLMCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: VLMCFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VLMCFG` reader - Configuration"]
pub type VLMCFG_R = crate::attiny412pac::FieldReader<u8, VLMCFG_A>;
impl VLMCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VLMCFG_A> {
        match self.bits {
            0 => Some(VLMCFG_A::BELOW),
            1 => Some(VLMCFG_A::ABOVE),
            2 => Some(VLMCFG_A::CROSS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == VLMCFG_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == VLMCFG_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `CROSS`"]
    #[inline(always)]
    pub fn is_cross(&self) -> bool {
        *self == VLMCFG_A::CROSS
    }
}
#[doc = "Field `VLMCFG` writer - Configuration"]
pub type VLMCFG_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, INTCTRL_SPEC, u8, VLMCFG_A, 2, O>;
impl<'a, const O: u8> VLMCFG_W<'a, O> {
    #[doc = "Interrupt when supply goes below VLM level"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(VLMCFG_A::BELOW)
    }
    #[doc = "Interrupt when supply goes above VLM level"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(VLMCFG_A::ABOVE)
    }
    #[doc = "Interrupt when supply crosses VLM level"]
    #[inline(always)]
    pub fn cross(self) -> &'a mut W {
        self.variant(VLMCFG_A::CROSS)
    }
}
impl R {
    #[doc = "Bit 0 - voltage level monitor interrrupt enable"]
    #[inline(always)]
    pub fn vlmie(&self) -> VLMIE_R {
        VLMIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configuration"]
    #[inline(always)]
    pub fn vlmcfg(&self) -> VLMCFG_R {
        VLMCFG_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - voltage level monitor interrrupt enable"]
    #[inline(always)]
    pub fn vlmie(&mut self) -> VLMIE_W<0> {
        VLMIE_W::new(self)
    }
    #[doc = "Bits 1:2 - Configuration"]
    #[inline(always)]
    pub fn vlmcfg(&mut self) -> VLMCFG_W<1> {
        VLMCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage level monitor interrupt Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intctrl](index.html) module"]
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
