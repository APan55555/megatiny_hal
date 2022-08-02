#[doc = "Register `WDTCFG` reader"]
pub struct R(crate::R<WDTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCFG` writer"]
pub struct W(crate::W<WDTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCFG_SPEC>;
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
impl From<crate::W<WDTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog Timeout Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERIOD_A {
    #[doc = "0: Watch-Dog timer Off"]
    OFF = 0,
    #[doc = "1: 8 cycles (8ms)"]
    _8CLK = 1,
    #[doc = "2: 16 cycles (16ms)"]
    _16CLK = 2,
    #[doc = "3: 32 cycles (32ms)"]
    _32CLK = 3,
    #[doc = "4: 64 cycles (64ms)"]
    _64CLK = 4,
    #[doc = "5: 128 cycles (0.128s)"]
    _128CLK = 5,
    #[doc = "6: 256 cycles (0.256s)"]
    _256CLK = 6,
    #[doc = "7: 512 cycles (0.512s)"]
    _512CLK = 7,
    #[doc = "8: 1K cycles (1.0s)"]
    _1KCLK = 8,
    #[doc = "9: 2K cycles (2.0s)"]
    _2KCLK = 9,
    #[doc = "10: 4K cycles (4.1s)"]
    _4KCLK = 10,
    #[doc = "11: 8K cycles (8.2s)"]
    _8KCLK = 11,
}
impl From<PERIOD_A> for u8 {
    #[inline(always)]
    fn from(variant: PERIOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PERIOD` reader - Watchdog Timeout Period"]
pub type PERIOD_R = crate::FieldReader<u8, PERIOD_A>;
impl PERIOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PERIOD_A> {
        match self.bits {
            0 => Some(PERIOD_A::OFF),
            1 => Some(PERIOD_A::_8CLK),
            2 => Some(PERIOD_A::_16CLK),
            3 => Some(PERIOD_A::_32CLK),
            4 => Some(PERIOD_A::_64CLK),
            5 => Some(PERIOD_A::_128CLK),
            6 => Some(PERIOD_A::_256CLK),
            7 => Some(PERIOD_A::_512CLK),
            8 => Some(PERIOD_A::_1KCLK),
            9 => Some(PERIOD_A::_2KCLK),
            10 => Some(PERIOD_A::_4KCLK),
            11 => Some(PERIOD_A::_8KCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == PERIOD_A::OFF
    }
    #[doc = "Checks if the value of the field is `_8CLK`"]
    #[inline(always)]
    pub fn is_8clk(&self) -> bool {
        *self == PERIOD_A::_8CLK
    }
    #[doc = "Checks if the value of the field is `_16CLK`"]
    #[inline(always)]
    pub fn is_16clk(&self) -> bool {
        *self == PERIOD_A::_16CLK
    }
    #[doc = "Checks if the value of the field is `_32CLK`"]
    #[inline(always)]
    pub fn is_32clk(&self) -> bool {
        *self == PERIOD_A::_32CLK
    }
    #[doc = "Checks if the value of the field is `_64CLK`"]
    #[inline(always)]
    pub fn is_64clk(&self) -> bool {
        *self == PERIOD_A::_64CLK
    }
    #[doc = "Checks if the value of the field is `_128CLK`"]
    #[inline(always)]
    pub fn is_128clk(&self) -> bool {
        *self == PERIOD_A::_128CLK
    }
    #[doc = "Checks if the value of the field is `_256CLK`"]
    #[inline(always)]
    pub fn is_256clk(&self) -> bool {
        *self == PERIOD_A::_256CLK
    }
    #[doc = "Checks if the value of the field is `_512CLK`"]
    #[inline(always)]
    pub fn is_512clk(&self) -> bool {
        *self == PERIOD_A::_512CLK
    }
    #[doc = "Checks if the value of the field is `_1KCLK`"]
    #[inline(always)]
    pub fn is_1kclk(&self) -> bool {
        *self == PERIOD_A::_1KCLK
    }
    #[doc = "Checks if the value of the field is `_2KCLK`"]
    #[inline(always)]
    pub fn is_2kclk(&self) -> bool {
        *self == PERIOD_A::_2KCLK
    }
    #[doc = "Checks if the value of the field is `_4KCLK`"]
    #[inline(always)]
    pub fn is_4kclk(&self) -> bool {
        *self == PERIOD_A::_4KCLK
    }
    #[doc = "Checks if the value of the field is `_8KCLK`"]
    #[inline(always)]
    pub fn is_8kclk(&self) -> bool {
        *self == PERIOD_A::_8KCLK
    }
}
#[doc = "Field `PERIOD` writer - Watchdog Timeout Period"]
pub type PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, WDTCFG_SPEC, u8, PERIOD_A, 4, O>;
impl<'a, const O: u8> PERIOD_W<'a, O> {
    #[doc = "Watch-Dog timer Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(PERIOD_A::OFF)
    }
    #[doc = "8 cycles (8ms)"]
    #[inline(always)]
    pub fn _8clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_8CLK)
    }
    #[doc = "16 cycles (16ms)"]
    #[inline(always)]
    pub fn _16clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_16CLK)
    }
    #[doc = "32 cycles (32ms)"]
    #[inline(always)]
    pub fn _32clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_32CLK)
    }
    #[doc = "64 cycles (64ms)"]
    #[inline(always)]
    pub fn _64clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_64CLK)
    }
    #[doc = "128 cycles (0.128s)"]
    #[inline(always)]
    pub fn _128clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_128CLK)
    }
    #[doc = "256 cycles (0.256s)"]
    #[inline(always)]
    pub fn _256clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_256CLK)
    }
    #[doc = "512 cycles (0.512s)"]
    #[inline(always)]
    pub fn _512clk(self) -> &'a mut W {
        self.variant(PERIOD_A::_512CLK)
    }
    #[doc = "1K cycles (1.0s)"]
    #[inline(always)]
    pub fn _1kclk(self) -> &'a mut W {
        self.variant(PERIOD_A::_1KCLK)
    }
    #[doc = "2K cycles (2.0s)"]
    #[inline(always)]
    pub fn _2kclk(self) -> &'a mut W {
        self.variant(PERIOD_A::_2KCLK)
    }
    #[doc = "4K cycles (4.1s)"]
    #[inline(always)]
    pub fn _4kclk(self) -> &'a mut W {
        self.variant(PERIOD_A::_4KCLK)
    }
    #[doc = "8K cycles (8.2s)"]
    #[inline(always)]
    pub fn _8kclk(self) -> &'a mut W {
        self.variant(PERIOD_A::_8KCLK)
    }
}
#[doc = "Watchdog Window Timeout Period\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINDOW_A {
    #[doc = "0: Window mode off"]
    OFF = 0,
    #[doc = "1: 8 cycles (8ms)"]
    _8CLK = 1,
    #[doc = "2: 16 cycles (16ms)"]
    _16CLK = 2,
    #[doc = "3: 32 cycles (32ms)"]
    _32CLK = 3,
    #[doc = "4: 64 cycles (64ms)"]
    _64CLK = 4,
    #[doc = "5: 128 cycles (0.128s)"]
    _128CLK = 5,
    #[doc = "6: 256 cycles (0.256s)"]
    _256CLK = 6,
    #[doc = "7: 512 cycles (0.512s)"]
    _512CLK = 7,
    #[doc = "8: 1K cycles (1.0s)"]
    _1KCLK = 8,
    #[doc = "9: 2K cycles (2.0s)"]
    _2KCLK = 9,
    #[doc = "10: 4K cycles (4.1s)"]
    _4KCLK = 10,
    #[doc = "11: 8K cycles (8.2s)"]
    _8KCLK = 11,
}
impl From<WINDOW_A> for u8 {
    #[inline(always)]
    fn from(variant: WINDOW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINDOW` reader - Watchdog Window Timeout Period"]
pub type WINDOW_R = crate::FieldReader<u8, WINDOW_A>;
impl WINDOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINDOW_A> {
        match self.bits {
            0 => Some(WINDOW_A::OFF),
            1 => Some(WINDOW_A::_8CLK),
            2 => Some(WINDOW_A::_16CLK),
            3 => Some(WINDOW_A::_32CLK),
            4 => Some(WINDOW_A::_64CLK),
            5 => Some(WINDOW_A::_128CLK),
            6 => Some(WINDOW_A::_256CLK),
            7 => Some(WINDOW_A::_512CLK),
            8 => Some(WINDOW_A::_1KCLK),
            9 => Some(WINDOW_A::_2KCLK),
            10 => Some(WINDOW_A::_4KCLK),
            11 => Some(WINDOW_A::_8KCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == WINDOW_A::OFF
    }
    #[doc = "Checks if the value of the field is `_8CLK`"]
    #[inline(always)]
    pub fn is_8clk(&self) -> bool {
        *self == WINDOW_A::_8CLK
    }
    #[doc = "Checks if the value of the field is `_16CLK`"]
    #[inline(always)]
    pub fn is_16clk(&self) -> bool {
        *self == WINDOW_A::_16CLK
    }
    #[doc = "Checks if the value of the field is `_32CLK`"]
    #[inline(always)]
    pub fn is_32clk(&self) -> bool {
        *self == WINDOW_A::_32CLK
    }
    #[doc = "Checks if the value of the field is `_64CLK`"]
    #[inline(always)]
    pub fn is_64clk(&self) -> bool {
        *self == WINDOW_A::_64CLK
    }
    #[doc = "Checks if the value of the field is `_128CLK`"]
    #[inline(always)]
    pub fn is_128clk(&self) -> bool {
        *self == WINDOW_A::_128CLK
    }
    #[doc = "Checks if the value of the field is `_256CLK`"]
    #[inline(always)]
    pub fn is_256clk(&self) -> bool {
        *self == WINDOW_A::_256CLK
    }
    #[doc = "Checks if the value of the field is `_512CLK`"]
    #[inline(always)]
    pub fn is_512clk(&self) -> bool {
        *self == WINDOW_A::_512CLK
    }
    #[doc = "Checks if the value of the field is `_1KCLK`"]
    #[inline(always)]
    pub fn is_1kclk(&self) -> bool {
        *self == WINDOW_A::_1KCLK
    }
    #[doc = "Checks if the value of the field is `_2KCLK`"]
    #[inline(always)]
    pub fn is_2kclk(&self) -> bool {
        *self == WINDOW_A::_2KCLK
    }
    #[doc = "Checks if the value of the field is `_4KCLK`"]
    #[inline(always)]
    pub fn is_4kclk(&self) -> bool {
        *self == WINDOW_A::_4KCLK
    }
    #[doc = "Checks if the value of the field is `_8KCLK`"]
    #[inline(always)]
    pub fn is_8kclk(&self) -> bool {
        *self == WINDOW_A::_8KCLK
    }
}
#[doc = "Field `WINDOW` writer - Watchdog Window Timeout Period"]
pub type WINDOW_W<'a, const O: u8> = crate::FieldWriter<'a, u8, WDTCFG_SPEC, u8, WINDOW_A, 4, O>;
impl<'a, const O: u8> WINDOW_W<'a, O> {
    #[doc = "Window mode off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(WINDOW_A::OFF)
    }
    #[doc = "8 cycles (8ms)"]
    #[inline(always)]
    pub fn _8clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_8CLK)
    }
    #[doc = "16 cycles (16ms)"]
    #[inline(always)]
    pub fn _16clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_16CLK)
    }
    #[doc = "32 cycles (32ms)"]
    #[inline(always)]
    pub fn _32clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_32CLK)
    }
    #[doc = "64 cycles (64ms)"]
    #[inline(always)]
    pub fn _64clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_64CLK)
    }
    #[doc = "128 cycles (0.128s)"]
    #[inline(always)]
    pub fn _128clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_128CLK)
    }
    #[doc = "256 cycles (0.256s)"]
    #[inline(always)]
    pub fn _256clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_256CLK)
    }
    #[doc = "512 cycles (0.512s)"]
    #[inline(always)]
    pub fn _512clk(self) -> &'a mut W {
        self.variant(WINDOW_A::_512CLK)
    }
    #[doc = "1K cycles (1.0s)"]
    #[inline(always)]
    pub fn _1kclk(self) -> &'a mut W {
        self.variant(WINDOW_A::_1KCLK)
    }
    #[doc = "2K cycles (2.0s)"]
    #[inline(always)]
    pub fn _2kclk(self) -> &'a mut W {
        self.variant(WINDOW_A::_2KCLK)
    }
    #[doc = "4K cycles (4.1s)"]
    #[inline(always)]
    pub fn _4kclk(self) -> &'a mut W {
        self.variant(WINDOW_A::_4KCLK)
    }
    #[doc = "8K cycles (8.2s)"]
    #[inline(always)]
    pub fn _8kclk(self) -> &'a mut W {
        self.variant(WINDOW_A::_8KCLK)
    }
}
impl R {
    #[doc = "Bits 0:3 - Watchdog Timeout Period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Watchdog Window Timeout Period"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Watchdog Timeout Period"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<0> {
        PERIOD_W::new(self)
    }
    #[doc = "Bits 4:7 - Watchdog Window Timeout Period"]
    #[inline(always)]
    pub fn window(&mut self) -> WINDOW_W<4> {
        WINDOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtcfg](index.html) module"]
pub struct WDTCFG_SPEC;
impl crate::RegisterSpec for WDTCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wdtcfg::R](R) reader structure"]
impl crate::Readable for WDTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtcfg::W](W) writer structure"]
impl crate::Writable for WDTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCFG to value 0"]
impl crate::Resettable for WDTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
