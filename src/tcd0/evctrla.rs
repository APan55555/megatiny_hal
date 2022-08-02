#[doc = "Register `EVCTRLA` reader"]
pub struct R(crate::R<EVCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTRLA` writer"]
pub struct W(crate::W<EVCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTRLA_SPEC>;
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
impl From<crate::W<EVCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGEI` reader - Trigger event enable"]
pub type TRIGEI_R = crate::BitReader<bool>;
#[doc = "Field `TRIGEI` writer - Trigger event enable"]
pub type TRIGEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRLA_SPEC, bool, O>;
#[doc = "event action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACTION_A {
    #[doc = "0: Event trigger a fault"]
    FAULT = 0,
    #[doc = "1: Event trigger a fault and capture"]
    CAPTURE = 1,
}
impl From<ACTION_A> for bool {
    #[inline(always)]
    fn from(variant: ACTION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACTION` reader - event action"]
pub type ACTION_R = crate::BitReader<ACTION_A>;
impl ACTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTION_A {
        match self.bits {
            false => ACTION_A::FAULT,
            true => ACTION_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == ACTION_A::FAULT
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == ACTION_A::CAPTURE
    }
}
#[doc = "Field `ACTION` writer - event action"]
pub type ACTION_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRLA_SPEC, ACTION_A, O>;
impl<'a, const O: u8> ACTION_W<'a, O> {
    #[doc = "Event trigger a fault"]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(ACTION_A::FAULT)
    }
    #[doc = "Event trigger a fault and capture"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(ACTION_A::CAPTURE)
    }
}
#[doc = "edge select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EDGE_A {
    #[doc = "0: The falling edge or low level of event generates retrigger or fault action"]
    FALL_LOW = 0,
    #[doc = "1: The rising edge or high level of event generates retrigger or fault action"]
    RISE_HIGH = 1,
}
impl From<EDGE_A> for bool {
    #[inline(always)]
    fn from(variant: EDGE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - edge select"]
pub type EDGE_R = crate::BitReader<EDGE_A>;
impl EDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE_A {
        match self.bits {
            false => EDGE_A::FALL_LOW,
            true => EDGE_A::RISE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `FALL_LOW`"]
    #[inline(always)]
    pub fn is_fall_low(&self) -> bool {
        *self == EDGE_A::FALL_LOW
    }
    #[doc = "Checks if the value of the field is `RISE_HIGH`"]
    #[inline(always)]
    pub fn is_rise_high(&self) -> bool {
        *self == EDGE_A::RISE_HIGH
    }
}
#[doc = "Field `EDGE` writer - edge select"]
pub type EDGE_W<'a, const O: u8> = crate::BitWriter<'a, u8, EVCTRLA_SPEC, EDGE_A, O>;
impl<'a, const O: u8> EDGE_W<'a, O> {
    #[doc = "The falling edge or low level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn fall_low(self) -> &'a mut W {
        self.variant(EDGE_A::FALL_LOW)
    }
    #[doc = "The rising edge or high level of event generates retrigger or fault action"]
    #[inline(always)]
    pub fn rise_high(self) -> &'a mut W {
        self.variant(EDGE_A::RISE_HIGH)
    }
}
#[doc = "event config\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFG_A {
    #[doc = "0: Neither Filter nor Asynchronous Event is enabled"]
    NEITHER = 0,
    #[doc = "1: Input Capture Noise Cancellation Filter enabled"]
    FILTER = 1,
    #[doc = "2: Asynchronous Event output qualification enabled"]
    ASYNC = 2,
}
impl From<CFG_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFG` reader - event config"]
pub type CFG_R = crate::FieldReader<u8, CFG_A>;
impl CFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFG_A> {
        match self.bits {
            0 => Some(CFG_A::NEITHER),
            1 => Some(CFG_A::FILTER),
            2 => Some(CFG_A::ASYNC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NEITHER`"]
    #[inline(always)]
    pub fn is_neither(&self) -> bool {
        *self == CFG_A::NEITHER
    }
    #[doc = "Checks if the value of the field is `FILTER`"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == CFG_A::FILTER
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == CFG_A::ASYNC
    }
}
#[doc = "Field `CFG` writer - event config"]
pub type CFG_W<'a, const O: u8> = crate::FieldWriter<'a, u8, EVCTRLA_SPEC, u8, CFG_A, 2, O>;
impl<'a, const O: u8> CFG_W<'a, O> {
    #[doc = "Neither Filter nor Asynchronous Event is enabled"]
    #[inline(always)]
    pub fn neither(self) -> &'a mut W {
        self.variant(CFG_A::NEITHER)
    }
    #[doc = "Input Capture Noise Cancellation Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut W {
        self.variant(CFG_A::FILTER)
    }
    #[doc = "Asynchronous Event output qualification enabled"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(CFG_A::ASYNC)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger event enable"]
    #[inline(always)]
    pub fn trigei(&self) -> TRIGEI_R {
        TRIGEI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - event action"]
    #[inline(always)]
    pub fn action(&self) -> ACTION_R {
        ACTION_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - edge select"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - event config"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger event enable"]
    #[inline(always)]
    pub fn trigei(&mut self) -> TRIGEI_W<0> {
        TRIGEI_W::new(self)
    }
    #[doc = "Bit 2 - event action"]
    #[inline(always)]
    pub fn action(&mut self) -> ACTION_W<2> {
        ACTION_W::new(self)
    }
    #[doc = "Bit 4 - edge select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W<4> {
        EDGE_W::new(self)
    }
    #[doc = "Bits 6:7 - event config"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W<6> {
        CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EVCTRLA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctrla](index.html) module"]
pub struct EVCTRLA_SPEC;
impl crate::RegisterSpec for EVCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [evctrla::R](R) reader structure"]
impl crate::Readable for EVCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctrla::W](W) writer structure"]
impl crate::Writable for EVCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVCTRLA to value 0"]
impl crate::Resettable for EVCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
