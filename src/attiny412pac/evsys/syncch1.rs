#[doc = "Register `SYNCCH1` reader"]
pub struct R(crate::attiny412pac::R<SYNCCH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SYNCCH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SYNCCH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SYNCCH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNCCH1` writer"]
pub struct W(crate::attiny412pac::W<SYNCCH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SYNCCH1_SPEC>;
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
impl From<crate::attiny412pac::W<SYNCCH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SYNCCH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Synchronous Channel 1 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SYNCCH1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Timer/Counter B0"]
    TCB0 = 1,
    #[doc = "2: Timer/Counter A0 overflow"]
    TCA0_OVF_LUNF = 2,
    #[doc = "3: Timer/Counter A0 underflow high byte (split mode)"]
    TCA0_HUNF = 3,
    #[doc = "4: Timer/Counter A0 compare 0"]
    TCA0_CMP0 = 4,
    #[doc = "5: Timer/Counter A0 compare 1"]
    TCA0_CMP1 = 5,
    #[doc = "6: Timer/Counter A0 compare 2"]
    TCA0_CMP2 = 6,
    #[doc = "8: Synchronous Event from Pin PB0"]
    PORTB_PIN0 = 8,
    #[doc = "9: Synchronous Event from Pin PB1"]
    PORTB_PIN1 = 9,
    #[doc = "10: Synchronous Event from Pin PB2"]
    PORTB_PIN2 = 10,
    #[doc = "11: Synchronous Event from Pin PB3"]
    PORTB_PIN3 = 11,
    #[doc = "12: Synchronous Event from Pin PB4"]
    PORTB_PIN4 = 12,
    #[doc = "13: Synchronous Event from Pin PB5"]
    PORTB_PIN5 = 13,
    #[doc = "14: Synchronous Event from Pin PB6"]
    PORTB_PIN6 = 14,
    #[doc = "15: Synchronous Event from Pin PB7"]
    PORTB_PIN7 = 15,
}
impl From<SYNCCH1_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCCH1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SYNCCH1` reader - Synchronous Channel 1 Generator Selection"]
pub type SYNCCH1_R = crate::attiny412pac::FieldReader<u8, SYNCCH1_A>;
impl SYNCCH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCCH1_A> {
        match self.bits {
            0 => Some(SYNCCH1_A::OFF),
            1 => Some(SYNCCH1_A::TCB0),
            2 => Some(SYNCCH1_A::TCA0_OVF_LUNF),
            3 => Some(SYNCCH1_A::TCA0_HUNF),
            4 => Some(SYNCCH1_A::TCA0_CMP0),
            5 => Some(SYNCCH1_A::TCA0_CMP1),
            6 => Some(SYNCCH1_A::TCA0_CMP2),
            8 => Some(SYNCCH1_A::PORTB_PIN0),
            9 => Some(SYNCCH1_A::PORTB_PIN1),
            10 => Some(SYNCCH1_A::PORTB_PIN2),
            11 => Some(SYNCCH1_A::PORTB_PIN3),
            12 => Some(SYNCCH1_A::PORTB_PIN4),
            13 => Some(SYNCCH1_A::PORTB_PIN5),
            14 => Some(SYNCCH1_A::PORTB_PIN6),
            15 => Some(SYNCCH1_A::PORTB_PIN7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SYNCCH1_A::OFF
    }
    #[doc = "Checks if the value of the field is `TCB0`"]
    #[inline(always)]
    pub fn is_tcb0(&self) -> bool {
        *self == SYNCCH1_A::TCB0
    }
    #[doc = "Checks if the value of the field is `TCA0_OVF_LUNF`"]
    #[inline(always)]
    pub fn is_tca0_ovf_lunf(&self) -> bool {
        *self == SYNCCH1_A::TCA0_OVF_LUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_HUNF`"]
    #[inline(always)]
    pub fn is_tca0_hunf(&self) -> bool {
        *self == SYNCCH1_A::TCA0_HUNF
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP0`"]
    #[inline(always)]
    pub fn is_tca0_cmp0(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP0
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP1`"]
    #[inline(always)]
    pub fn is_tca0_cmp1(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP1
    }
    #[doc = "Checks if the value of the field is `TCA0_CMP2`"]
    #[inline(always)]
    pub fn is_tca0_cmp2(&self) -> bool {
        *self == SYNCCH1_A::TCA0_CMP2
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN0`"]
    #[inline(always)]
    pub fn is_portb_pin0(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN0
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN1`"]
    #[inline(always)]
    pub fn is_portb_pin1(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN1
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN2`"]
    #[inline(always)]
    pub fn is_portb_pin2(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN2
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN3`"]
    #[inline(always)]
    pub fn is_portb_pin3(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN3
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN4`"]
    #[inline(always)]
    pub fn is_portb_pin4(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN4
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN5`"]
    #[inline(always)]
    pub fn is_portb_pin5(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN5
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN6`"]
    #[inline(always)]
    pub fn is_portb_pin6(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN6
    }
    #[doc = "Checks if the value of the field is `PORTB_PIN7`"]
    #[inline(always)]
    pub fn is_portb_pin7(&self) -> bool {
        *self == SYNCCH1_A::PORTB_PIN7
    }
}
#[doc = "Field `SYNCCH1` writer - Synchronous Channel 1 Generator Selection"]
pub type SYNCCH1_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, SYNCCH1_SPEC, u8, SYNCCH1_A, 8, O>;
impl<'a, const O: u8> SYNCCH1_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SYNCCH1_A::OFF)
    }
    #[doc = "Timer/Counter B0"]
    #[inline(always)]
    pub fn tcb0(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCB0)
    }
    #[doc = "Timer/Counter A0 overflow"]
    #[inline(always)]
    pub fn tca0_ovf_lunf(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCA0_OVF_LUNF)
    }
    #[doc = "Timer/Counter A0 underflow high byte (split mode)"]
    #[inline(always)]
    pub fn tca0_hunf(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCA0_HUNF)
    }
    #[doc = "Timer/Counter A0 compare 0"]
    #[inline(always)]
    pub fn tca0_cmp0(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCA0_CMP0)
    }
    #[doc = "Timer/Counter A0 compare 1"]
    #[inline(always)]
    pub fn tca0_cmp1(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCA0_CMP1)
    }
    #[doc = "Timer/Counter A0 compare 2"]
    #[inline(always)]
    pub fn tca0_cmp2(self) -> &'a mut W {
        self.variant(SYNCCH1_A::TCA0_CMP2)
    }
    #[doc = "Synchronous Event from Pin PB0"]
    #[inline(always)]
    pub fn portb_pin0(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN0)
    }
    #[doc = "Synchronous Event from Pin PB1"]
    #[inline(always)]
    pub fn portb_pin1(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN1)
    }
    #[doc = "Synchronous Event from Pin PB2"]
    #[inline(always)]
    pub fn portb_pin2(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN2)
    }
    #[doc = "Synchronous Event from Pin PB3"]
    #[inline(always)]
    pub fn portb_pin3(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN3)
    }
    #[doc = "Synchronous Event from Pin PB4"]
    #[inline(always)]
    pub fn portb_pin4(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN4)
    }
    #[doc = "Synchronous Event from Pin PB5"]
    #[inline(always)]
    pub fn portb_pin5(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN5)
    }
    #[doc = "Synchronous Event from Pin PB6"]
    #[inline(always)]
    pub fn portb_pin6(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN6)
    }
    #[doc = "Synchronous Event from Pin PB7"]
    #[inline(always)]
    pub fn portb_pin7(self) -> &'a mut W {
        self.variant(SYNCCH1_A::PORTB_PIN7)
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub fn syncch1(&self) -> SYNCCH1_R {
        SYNCCH1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronous Channel 1 Generator Selection"]
    #[inline(always)]
    pub fn syncch1(&mut self) -> SYNCCH1_W<0> {
        SYNCCH1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronous Channel 1 Generator Selection\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncch1](index.html) module"]
pub struct SYNCCH1_SPEC;
impl crate::attiny412pac::RegisterSpec for SYNCCH1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [syncch1::R](R) reader structure"]
impl crate::attiny412pac::Readable for SYNCCH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syncch1::W](W) writer structure"]
impl crate::attiny412pac::Writable for SYNCCH1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYNCCH1 to value 0"]
impl crate::attiny412pac::Resettable for SYNCCH1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
