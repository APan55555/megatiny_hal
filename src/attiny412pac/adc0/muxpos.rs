#[doc = "Register `MUXPOS` reader"]
pub struct R(crate::attiny412pac::R<MUXPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<MUXPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<MUXPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<MUXPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXPOS` writer"]
pub struct W(crate::attiny412pac::W<MUXPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<MUXPOS_SPEC>;
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
impl From<crate::attiny412pac::W<MUXPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<MUXPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Analog Channel Selection Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUXPOS_A {
    #[doc = "0: ADC input pin 0"]
    AIN0 = 0,
    #[doc = "1: ADC input pin 1"]
    AIN1 = 1,
    #[doc = "2: ADC input pin 2"]
    AIN2 = 2,
    #[doc = "3: ADC input pin 3"]
    AIN3 = 3,
    #[doc = "4: ADC input pin 4"]
    AIN4 = 4,
    #[doc = "5: ADC input pin 5"]
    AIN5 = 5,
    #[doc = "6: ADC input pin 6"]
    AIN6 = 6,
    #[doc = "7: ADC input pin 7"]
    AIN7 = 7,
    #[doc = "8: ADC input pin 8"]
    AIN8 = 8,
    #[doc = "9: ADC input pin 9"]
    AIN9 = 9,
    #[doc = "10: ADC input pin 10"]
    AIN10 = 10,
    #[doc = "11: ADC input pin 11"]
    AIN11 = 11,
    #[doc = "28: DAC0"]
    DAC0 = 28,
    #[doc = "29: Internal Ref"]
    INTREF = 29,
    #[doc = "30: Temp sensor"]
    TEMPSENSE = 30,
    #[doc = "31: GND"]
    GND = 31,
}
impl From<MUXPOS_A> for u8 {
    #[inline(always)]
    fn from(variant: MUXPOS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MUXPOS` reader - Analog Channel Selection Bits"]
pub type MUXPOS_R = crate::attiny412pac::FieldReader<u8, MUXPOS_A>;
impl MUXPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MUXPOS_A> {
        match self.bits {
            0 => Some(MUXPOS_A::AIN0),
            1 => Some(MUXPOS_A::AIN1),
            2 => Some(MUXPOS_A::AIN2),
            3 => Some(MUXPOS_A::AIN3),
            4 => Some(MUXPOS_A::AIN4),
            5 => Some(MUXPOS_A::AIN5),
            6 => Some(MUXPOS_A::AIN6),
            7 => Some(MUXPOS_A::AIN7),
            8 => Some(MUXPOS_A::AIN8),
            9 => Some(MUXPOS_A::AIN9),
            10 => Some(MUXPOS_A::AIN10),
            11 => Some(MUXPOS_A::AIN11),
            28 => Some(MUXPOS_A::DAC0),
            29 => Some(MUXPOS_A::INTREF),
            30 => Some(MUXPOS_A::TEMPSENSE),
            31 => Some(MUXPOS_A::GND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AIN0`"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == MUXPOS_A::AIN0
    }
    #[doc = "Checks if the value of the field is `AIN1`"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == MUXPOS_A::AIN1
    }
    #[doc = "Checks if the value of the field is `AIN2`"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == MUXPOS_A::AIN2
    }
    #[doc = "Checks if the value of the field is `AIN3`"]
    #[inline(always)]
    pub fn is_ain3(&self) -> bool {
        *self == MUXPOS_A::AIN3
    }
    #[doc = "Checks if the value of the field is `AIN4`"]
    #[inline(always)]
    pub fn is_ain4(&self) -> bool {
        *self == MUXPOS_A::AIN4
    }
    #[doc = "Checks if the value of the field is `AIN5`"]
    #[inline(always)]
    pub fn is_ain5(&self) -> bool {
        *self == MUXPOS_A::AIN5
    }
    #[doc = "Checks if the value of the field is `AIN6`"]
    #[inline(always)]
    pub fn is_ain6(&self) -> bool {
        *self == MUXPOS_A::AIN6
    }
    #[doc = "Checks if the value of the field is `AIN7`"]
    #[inline(always)]
    pub fn is_ain7(&self) -> bool {
        *self == MUXPOS_A::AIN7
    }
    #[doc = "Checks if the value of the field is `AIN8`"]
    #[inline(always)]
    pub fn is_ain8(&self) -> bool {
        *self == MUXPOS_A::AIN8
    }
    #[doc = "Checks if the value of the field is `AIN9`"]
    #[inline(always)]
    pub fn is_ain9(&self) -> bool {
        *self == MUXPOS_A::AIN9
    }
    #[doc = "Checks if the value of the field is `AIN10`"]
    #[inline(always)]
    pub fn is_ain10(&self) -> bool {
        *self == MUXPOS_A::AIN10
    }
    #[doc = "Checks if the value of the field is `AIN11`"]
    #[inline(always)]
    pub fn is_ain11(&self) -> bool {
        *self == MUXPOS_A::AIN11
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == MUXPOS_A::DAC0
    }
    #[doc = "Checks if the value of the field is `INTREF`"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == MUXPOS_A::INTREF
    }
    #[doc = "Checks if the value of the field is `TEMPSENSE`"]
    #[inline(always)]
    pub fn is_tempsense(&self) -> bool {
        *self == MUXPOS_A::TEMPSENSE
    }
    #[doc = "Checks if the value of the field is `GND`"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == MUXPOS_A::GND
    }
}
#[doc = "Field `MUXPOS` writer - Analog Channel Selection Bits"]
pub type MUXPOS_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, MUXPOS_SPEC, u8, MUXPOS_A, 5, O>;
impl<'a, const O: u8> MUXPOS_W<'a, O> {
    #[doc = "ADC input pin 0"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN0)
    }
    #[doc = "ADC input pin 1"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN1)
    }
    #[doc = "ADC input pin 2"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN2)
    }
    #[doc = "ADC input pin 3"]
    #[inline(always)]
    pub fn ain3(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN3)
    }
    #[doc = "ADC input pin 4"]
    #[inline(always)]
    pub fn ain4(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN4)
    }
    #[doc = "ADC input pin 5"]
    #[inline(always)]
    pub fn ain5(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN5)
    }
    #[doc = "ADC input pin 6"]
    #[inline(always)]
    pub fn ain6(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN6)
    }
    #[doc = "ADC input pin 7"]
    #[inline(always)]
    pub fn ain7(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN7)
    }
    #[doc = "ADC input pin 8"]
    #[inline(always)]
    pub fn ain8(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN8)
    }
    #[doc = "ADC input pin 9"]
    #[inline(always)]
    pub fn ain9(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN9)
    }
    #[doc = "ADC input pin 10"]
    #[inline(always)]
    pub fn ain10(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN10)
    }
    #[doc = "ADC input pin 11"]
    #[inline(always)]
    pub fn ain11(self) -> &'a mut W {
        self.variant(MUXPOS_A::AIN11)
    }
    #[doc = "DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(MUXPOS_A::DAC0)
    }
    #[doc = "Internal Ref"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut W {
        self.variant(MUXPOS_A::INTREF)
    }
    #[doc = "Temp sensor"]
    #[inline(always)]
    pub fn tempsense(self) -> &'a mut W {
        self.variant(MUXPOS_A::TEMPSENSE)
    }
    #[doc = "GND"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut W {
        self.variant(MUXPOS_A::GND)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn muxpos(&self) -> MUXPOS_R {
        MUXPOS_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog Channel Selection Bits"]
    #[inline(always)]
    pub fn muxpos(&mut self) -> MUXPOS_W<0> {
        MUXPOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Positive mux input\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxpos](index.html) module"]
pub struct MUXPOS_SPEC;
impl crate::attiny412pac::RegisterSpec for MUXPOS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxpos::R](R) reader structure"]
impl crate::attiny412pac::Readable for MUXPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxpos::W](W) writer structure"]
impl crate::attiny412pac::Writable for MUXPOS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MUXPOS to value 0"]
impl crate::attiny412pac::Resettable for MUXPOS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
