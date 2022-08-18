#[doc = "Register `MCTRLA` reader"]
pub struct R(crate::attiny412pac::R<MCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<MCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<MCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<MCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRLA` writer"]
pub struct W(crate::attiny412pac::W<MCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<MCTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<MCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<MCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable TWI Master"]
pub type ENABLE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable TWI Master"]
pub type ENABLE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `SMEN` reader - Smart Mode Enable"]
pub type SMEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `SMEN` writer - Smart Mode Enable"]
pub type SMEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Inactive Bus Timeout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TIMEOUT_A {
    #[doc = "0: Bus Timeout Disabled"]
    DISABLED = 0,
    #[doc = "1: 50 Microseconds"]
    _50US = 1,
    #[doc = "2: 100 Microseconds"]
    _100US = 2,
    #[doc = "3: 200 Microseconds"]
    _200US = 3,
}
impl From<TIMEOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TIMEOUT` reader - Inactive Bus Timeout"]
pub type TIMEOUT_R = crate::attiny412pac::FieldReader<u8, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            0 => TIMEOUT_A::DISABLED,
            1 => TIMEOUT_A::_50US,
            2 => TIMEOUT_A::_100US,
            3 => TIMEOUT_A::_200US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMEOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `_50US`"]
    #[inline(always)]
    pub fn is_50us(&self) -> bool {
        *self == TIMEOUT_A::_50US
    }
    #[doc = "Checks if the value of the field is `_100US`"]
    #[inline(always)]
    pub fn is_100us(&self) -> bool {
        *self == TIMEOUT_A::_100US
    }
    #[doc = "Checks if the value of the field is `_200US`"]
    #[inline(always)]
    pub fn is_200us(&self) -> bool {
        *self == TIMEOUT_A::_200US
    }
}
#[doc = "Field `TIMEOUT` writer - Inactive Bus Timeout"]
pub type TIMEOUT_W<'a, const O: u8> =
    crate::attiny412pac::FieldWriterSafe<'a, u8, MCTRLA_SPEC, u8, TIMEOUT_A, 2, O>;
impl<'a, const O: u8> TIMEOUT_W<'a, O> {
    #[doc = "Bus Timeout Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMEOUT_A::DISABLED)
    }
    #[doc = "50 Microseconds"]
    #[inline(always)]
    pub fn _50us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_50US)
    }
    #[doc = "100 Microseconds"]
    #[inline(always)]
    pub fn _100us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_100US)
    }
    #[doc = "200 Microseconds"]
    #[inline(always)]
    pub fn _200us(self) -> &'a mut W {
        self.variant(TIMEOUT_A::_200US)
    }
}
#[doc = "Field `QCEN` reader - Quick Command Enable"]
pub type QCEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `QCEN` writer - Quick Command Enable"]
pub type QCEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `WIEN` reader - Write Interrupt Enable"]
pub type WIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `WIEN` writer - Write Interrupt Enable"]
pub type WIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
#[doc = "Field `RIEN` reader - Read Interrupt Enable"]
pub type RIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RIEN` writer - Read Interrupt Enable"]
pub type RIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, MCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable TWI Master"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Inactive Bus Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    pub fn qcen(&self) -> QCEN_R {
        QCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable TWI Master"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W<1> {
        SMEN_W::new(self)
    }
    #[doc = "Bits 2:3 - Inactive Bus Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<2> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 4 - Quick Command Enable"]
    #[inline(always)]
    pub fn qcen(&mut self) -> QCEN_W<4> {
        QCEN_W::new(self)
    }
    #[doc = "Bit 6 - Write Interrupt Enable"]
    #[inline(always)]
    pub fn wien(&mut self) -> WIEN_W<6> {
        WIEN_W::new(self)
    }
    #[doc = "Bit 7 - Read Interrupt Enable"]
    #[inline(always)]
    pub fn rien(&mut self) -> RIEN_W<7> {
        RIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrla](index.html) module"]
pub struct MCTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for MCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for MCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for MCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCTRLA to value 0"]
impl crate::attiny412pac::Resettable for MCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
