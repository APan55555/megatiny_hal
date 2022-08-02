#[doc = "Register `ASYNCCH0` reader"]
pub struct R(crate::R<ASYNCCH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASYNCCH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ASYNCCH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ASYNCCH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCCH0` writer"]
pub struct W(crate::W<ASYNCCH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASYNCCH0_SPEC>;
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
impl From<crate::W<ASYNCCH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ASYNCCH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Asynchronous Channel 0 Generator Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASYNCCH0_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Configurable Custom Logic LUT0"]
    CCL_LUT0 = 1,
    #[doc = "2: Configurable Custom Logic LUT1"]
    CCL_LUT1 = 2,
    #[doc = "3: Analog Comparator 0 out"]
    AC0_OUT = 3,
    #[doc = "4: Timer/Counter D0 compare B clear"]
    TCD0_CMPBCLR = 4,
    #[doc = "5: Timer/Counter D0 compare A set"]
    TCD0_CMPASET = 5,
    #[doc = "6: Timer/Counter D0 compare B set"]
    TCD0_CMPBSET = 6,
    #[doc = "7: Timer/Counter D0 program event"]
    TCD0_PROGEV = 7,
    #[doc = "8: Real Time Counter overflow"]
    RTC_OVF = 8,
    #[doc = "9: Real Time Counter compare"]
    RTC_CMP = 9,
    #[doc = "10: Asynchronous Event from Pin PA0"]
    PORTA_PIN0 = 10,
    #[doc = "11: Asynchronous Event from Pin PA1"]
    PORTA_PIN1 = 11,
    #[doc = "12: Asynchronous Event from Pin PA2"]
    PORTA_PIN2 = 12,
    #[doc = "13: Asynchronous Event from Pin PA3"]
    PORTA_PIN3 = 13,
    #[doc = "14: Asynchronous Event from Pin PA4"]
    PORTA_PIN4 = 14,
    #[doc = "15: Asynchronous Event from Pin PA5"]
    PORTA_PIN5 = 15,
    #[doc = "16: Asynchronous Event from Pin PA6"]
    PORTA_PIN6 = 16,
    #[doc = "17: Asynchronous Event from Pin PA7"]
    PORTA_PIN7 = 17,
    #[doc = "18: Unified Program and debug interface"]
    UPDI = 18,
}
impl From<ASYNCCH0_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCCH0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ASYNCCH0` reader - Asynchronous Channel 0 Generator Selection"]
pub type ASYNCCH0_R = crate::FieldReader<u8, ASYNCCH0_A>;
impl ASYNCCH0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCCH0_A> {
        match self.bits {
            0 => Some(ASYNCCH0_A::OFF),
            1 => Some(ASYNCCH0_A::CCL_LUT0),
            2 => Some(ASYNCCH0_A::CCL_LUT1),
            3 => Some(ASYNCCH0_A::AC0_OUT),
            4 => Some(ASYNCCH0_A::TCD0_CMPBCLR),
            5 => Some(ASYNCCH0_A::TCD0_CMPASET),
            6 => Some(ASYNCCH0_A::TCD0_CMPBSET),
            7 => Some(ASYNCCH0_A::TCD0_PROGEV),
            8 => Some(ASYNCCH0_A::RTC_OVF),
            9 => Some(ASYNCCH0_A::RTC_CMP),
            10 => Some(ASYNCCH0_A::PORTA_PIN0),
            11 => Some(ASYNCCH0_A::PORTA_PIN1),
            12 => Some(ASYNCCH0_A::PORTA_PIN2),
            13 => Some(ASYNCCH0_A::PORTA_PIN3),
            14 => Some(ASYNCCH0_A::PORTA_PIN4),
            15 => Some(ASYNCCH0_A::PORTA_PIN5),
            16 => Some(ASYNCCH0_A::PORTA_PIN6),
            17 => Some(ASYNCCH0_A::PORTA_PIN7),
            18 => Some(ASYNCCH0_A::UPDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCCH0_A::OFF
    }
    #[doc = "Checks if the value of the field is `CCL_LUT0`"]
    #[inline(always)]
    pub fn is_ccl_lut0(&self) -> bool {
        *self == ASYNCCH0_A::CCL_LUT0
    }
    #[doc = "Checks if the value of the field is `CCL_LUT1`"]
    #[inline(always)]
    pub fn is_ccl_lut1(&self) -> bool {
        *self == ASYNCCH0_A::CCL_LUT1
    }
    #[doc = "Checks if the value of the field is `AC0_OUT`"]
    #[inline(always)]
    pub fn is_ac0_out(&self) -> bool {
        *self == ASYNCCH0_A::AC0_OUT
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBCLR`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbclr(&self) -> bool {
        *self == ASYNCCH0_A::TCD0_CMPBCLR
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPASET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpaset(&self) -> bool {
        *self == ASYNCCH0_A::TCD0_CMPASET
    }
    #[doc = "Checks if the value of the field is `TCD0_CMPBSET`"]
    #[inline(always)]
    pub fn is_tcd0_cmpbset(&self) -> bool {
        *self == ASYNCCH0_A::TCD0_CMPBSET
    }
    #[doc = "Checks if the value of the field is `TCD0_PROGEV`"]
    #[inline(always)]
    pub fn is_tcd0_progev(&self) -> bool {
        *self == ASYNCCH0_A::TCD0_PROGEV
    }
    #[doc = "Checks if the value of the field is `RTC_OVF`"]
    #[inline(always)]
    pub fn is_rtc_ovf(&self) -> bool {
        *self == ASYNCCH0_A::RTC_OVF
    }
    #[doc = "Checks if the value of the field is `RTC_CMP`"]
    #[inline(always)]
    pub fn is_rtc_cmp(&self) -> bool {
        *self == ASYNCCH0_A::RTC_CMP
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN0`"]
    #[inline(always)]
    pub fn is_porta_pin0(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN0
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN1`"]
    #[inline(always)]
    pub fn is_porta_pin1(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN1
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN2`"]
    #[inline(always)]
    pub fn is_porta_pin2(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN2
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN3`"]
    #[inline(always)]
    pub fn is_porta_pin3(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN3
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN4`"]
    #[inline(always)]
    pub fn is_porta_pin4(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN4
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN5`"]
    #[inline(always)]
    pub fn is_porta_pin5(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN5
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN6`"]
    #[inline(always)]
    pub fn is_porta_pin6(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN6
    }
    #[doc = "Checks if the value of the field is `PORTA_PIN7`"]
    #[inline(always)]
    pub fn is_porta_pin7(&self) -> bool {
        *self == ASYNCCH0_A::PORTA_PIN7
    }
    #[doc = "Checks if the value of the field is `UPDI`"]
    #[inline(always)]
    pub fn is_updi(&self) -> bool {
        *self == ASYNCCH0_A::UPDI
    }
}
#[doc = "Field `ASYNCCH0` writer - Asynchronous Channel 0 Generator Selection"]
pub type ASYNCCH0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, ASYNCCH0_SPEC, u8, ASYNCCH0_A, 8, O>;
impl<'a, const O: u8> ASYNCCH0_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::OFF)
    }
    #[doc = "Configurable Custom Logic LUT0"]
    #[inline(always)]
    pub fn ccl_lut0(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::CCL_LUT0)
    }
    #[doc = "Configurable Custom Logic LUT1"]
    #[inline(always)]
    pub fn ccl_lut1(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::CCL_LUT1)
    }
    #[doc = "Analog Comparator 0 out"]
    #[inline(always)]
    pub fn ac0_out(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::AC0_OUT)
    }
    #[doc = "Timer/Counter D0 compare B clear"]
    #[inline(always)]
    pub fn tcd0_cmpbclr(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::TCD0_CMPBCLR)
    }
    #[doc = "Timer/Counter D0 compare A set"]
    #[inline(always)]
    pub fn tcd0_cmpaset(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::TCD0_CMPASET)
    }
    #[doc = "Timer/Counter D0 compare B set"]
    #[inline(always)]
    pub fn tcd0_cmpbset(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::TCD0_CMPBSET)
    }
    #[doc = "Timer/Counter D0 program event"]
    #[inline(always)]
    pub fn tcd0_progev(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::TCD0_PROGEV)
    }
    #[doc = "Real Time Counter overflow"]
    #[inline(always)]
    pub fn rtc_ovf(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::RTC_OVF)
    }
    #[doc = "Real Time Counter compare"]
    #[inline(always)]
    pub fn rtc_cmp(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::RTC_CMP)
    }
    #[doc = "Asynchronous Event from Pin PA0"]
    #[inline(always)]
    pub fn porta_pin0(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN0)
    }
    #[doc = "Asynchronous Event from Pin PA1"]
    #[inline(always)]
    pub fn porta_pin1(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN1)
    }
    #[doc = "Asynchronous Event from Pin PA2"]
    #[inline(always)]
    pub fn porta_pin2(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN2)
    }
    #[doc = "Asynchronous Event from Pin PA3"]
    #[inline(always)]
    pub fn porta_pin3(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN3)
    }
    #[doc = "Asynchronous Event from Pin PA4"]
    #[inline(always)]
    pub fn porta_pin4(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN4)
    }
    #[doc = "Asynchronous Event from Pin PA5"]
    #[inline(always)]
    pub fn porta_pin5(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN5)
    }
    #[doc = "Asynchronous Event from Pin PA6"]
    #[inline(always)]
    pub fn porta_pin6(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN6)
    }
    #[doc = "Asynchronous Event from Pin PA7"]
    #[inline(always)]
    pub fn porta_pin7(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::PORTA_PIN7)
    }
    #[doc = "Unified Program and debug interface"]
    #[inline(always)]
    pub fn updi(self) -> &'a mut W {
        self.variant(ASYNCCH0_A::UPDI)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous Channel 0 Generator Selection"]
    #[inline(always)]
    pub fn asyncch0(&self) -> ASYNCCH0_R {
        ASYNCCH0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous Channel 0 Generator Selection"]
    #[inline(always)]
    pub fn asyncch0(&mut self) -> ASYNCCH0_W<0> {
        ASYNCCH0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous Channel 0 Generator Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncch0](index.html) module"]
pub struct ASYNCCH0_SPEC;
impl crate::RegisterSpec for ASYNCCH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncch0::R](R) reader structure"]
impl crate::Readable for ASYNCCH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncch0::W](W) writer structure"]
impl crate::Writable for ASYNCCH0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCCH0 to value 0"]
impl crate::Resettable for ASYNCCH0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
