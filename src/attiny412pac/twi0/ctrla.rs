#[doc = "Register `CTRLA` reader"]
pub struct R(crate::attiny412pac::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::attiny412pac::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMPEN` reader - FM Plus Enable"]
pub type FMPEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `FMPEN` writer - FM Plus Enable"]
pub type FMPEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "SDA Hold Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDAHOLD_A {
    #[doc = "0: SDA hold time off"]
    OFF = 0,
    #[doc = "1: Typical 50ns hold time"]
    _50NS = 1,
    #[doc = "2: Typical 300ns hold time"]
    _300NS = 2,
    #[doc = "3: Typical 500ns hold time"]
    _500NS = 3,
}
impl From<SDAHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: SDAHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SDAHOLD` reader - SDA Hold Time"]
pub type SDAHOLD_R = crate::attiny412pac::FieldReader<u8, SDAHOLD_A>;
impl SDAHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDAHOLD_A {
        match self.bits {
            0 => SDAHOLD_A::OFF,
            1 => SDAHOLD_A::_50NS,
            2 => SDAHOLD_A::_300NS,
            3 => SDAHOLD_A::_500NS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == SDAHOLD_A::OFF
    }
    #[doc = "Checks if the value of the field is `_50NS`"]
    #[inline(always)]
    pub fn is_50ns(&self) -> bool {
        *self == SDAHOLD_A::_50NS
    }
    #[doc = "Checks if the value of the field is `_300NS`"]
    #[inline(always)]
    pub fn is_300ns(&self) -> bool {
        *self == SDAHOLD_A::_300NS
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == SDAHOLD_A::_500NS
    }
}
#[doc = "Field `SDAHOLD` writer - SDA Hold Time"]
pub type SDAHOLD_W<'a, const O: u8> =
    crate::attiny412pac::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, SDAHOLD_A, 2, O>;
impl<'a, const O: u8> SDAHOLD_W<'a, O> {
    #[doc = "SDA hold time off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(SDAHOLD_A::OFF)
    }
    #[doc = "Typical 50ns hold time"]
    #[inline(always)]
    pub fn _50ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_50NS)
    }
    #[doc = "Typical 300ns hold time"]
    #[inline(always)]
    pub fn _300ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_300NS)
    }
    #[doc = "Typical 500ns hold time"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(SDAHOLD_A::_500NS)
    }
}
#[doc = "SDA Setup Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDASETUP_A {
    #[doc = "0: SDA setup time is 4 clock cycles"]
    _4CYC = 0,
    #[doc = "1: SDA setup time is 8 clock cycles"]
    _8CYC = 1,
}
impl From<SDASETUP_A> for bool {
    #[inline(always)]
    fn from(variant: SDASETUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDASETUP` reader - SDA Setup Time"]
pub type SDASETUP_R = crate::attiny412pac::BitReader<SDASETUP_A>;
impl SDASETUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDASETUP_A {
        match self.bits {
            false => SDASETUP_A::_4CYC,
            true => SDASETUP_A::_8CYC,
        }
    }
    #[doc = "Checks if the value of the field is `_4CYC`"]
    #[inline(always)]
    pub fn is_4cyc(&self) -> bool {
        *self == SDASETUP_A::_4CYC
    }
    #[doc = "Checks if the value of the field is `_8CYC`"]
    #[inline(always)]
    pub fn is_8cyc(&self) -> bool {
        *self == SDASETUP_A::_8CYC
    }
}
#[doc = "Field `SDASETUP` writer - SDA Setup Time"]
pub type SDASETUP_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLA_SPEC, SDASETUP_A, O>;
impl<'a, const O: u8> SDASETUP_W<'a, O> {
    #[doc = "SDA setup time is 4 clock cycles"]
    #[inline(always)]
    pub fn _4cyc(self) -> &'a mut W {
        self.variant(SDASETUP_A::_4CYC)
    }
    #[doc = "SDA setup time is 8 clock cycles"]
    #[inline(always)]
    pub fn _8cyc(self) -> &'a mut W {
        self.variant(SDASETUP_A::_8CYC)
    }
}
impl R {
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    pub fn fmpen(&self) -> FMPEN_R {
        FMPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&self) -> SDAHOLD_R {
        SDAHOLD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&self) -> SDASETUP_R {
        SDASETUP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FM Plus Enable"]
    #[inline(always)]
    pub fn fmpen(&mut self) -> FMPEN_W<1> {
        FMPEN_W::new(self)
    }
    #[doc = "Bits 2:3 - SDA Hold Time"]
    #[inline(always)]
    pub fn sdahold(&mut self) -> SDAHOLD_W<2> {
        SDAHOLD_W::new(self)
    }
    #[doc = "Bit 4 - SDA Setup Time"]
    #[inline(always)]
    pub fn sdasetup(&mut self) -> SDASETUP_W<4> {
        SDASETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::attiny412pac::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
