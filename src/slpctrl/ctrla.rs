#[doc = "Register `CTRLA` reader"]
pub struct R(crate::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLA_SPEC>;
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
impl From<crate::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEN` reader - Sleep enable"]
pub type SEN_R = crate::BitReader<bool>;
#[doc = "Field `SEN` writer - Sleep enable"]
pub type SEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Sleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMODE_A {
    #[doc = "0: Idle mode"]
    IDLE = 0,
    #[doc = "1: Standby Mode"]
    STDBY = 1,
    #[doc = "2: Power-down Mode"]
    PDOWN = 2,
}
impl From<SMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMODE` reader - Sleep mode"]
pub type SMODE_R = crate::FieldReader<u8, SMODE_A>;
impl SMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMODE_A> {
        match self.bits {
            0 => Some(SMODE_A::IDLE),
            1 => Some(SMODE_A::STDBY),
            2 => Some(SMODE_A::PDOWN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == SMODE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `STDBY`"]
    #[inline(always)]
    pub fn is_stdby(&self) -> bool {
        *self == SMODE_A::STDBY
    }
    #[doc = "Checks if the value of the field is `PDOWN`"]
    #[inline(always)]
    pub fn is_pdown(&self) -> bool {
        *self == SMODE_A::PDOWN
    }
}
#[doc = "Field `SMODE` writer - Sleep mode"]
pub type SMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, SMODE_A, 2, O>;
impl<'a, const O: u8> SMODE_W<'a, O> {
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(SMODE_A::IDLE)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn stdby(self) -> &'a mut W {
        self.variant(SMODE_A::STDBY)
    }
    #[doc = "Power-down Mode"]
    #[inline(always)]
    pub fn pdown(self) -> &'a mut W {
        self.variant(SMODE_A::PDOWN)
    }
}
impl R {
    #[doc = "Bit 0 - Sleep enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Sleep mode"]
    #[inline(always)]
    pub fn smode(&self) -> SMODE_R {
        SMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sleep enable"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W<0> {
        SEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Sleep mode"]
    #[inline(always)]
    pub fn smode(&mut self) -> SMODE_W<1> {
        SMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
