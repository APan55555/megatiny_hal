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
#[doc = "Field `ENABLE` reader - Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Hysteresis Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HYSMODE_A {
    #[doc = "0: No hysteresis"]
    OFF = 0,
    #[doc = "1: 10mV hysteresis"]
    _10MV = 1,
    #[doc = "2: 25mV hysteresis"]
    _25MV = 2,
    #[doc = "3: 50mV hysteresis"]
    _50MV = 3,
}
impl From<HYSMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HYSMODE` reader - Hysteresis Mode"]
pub type HYSMODE_R = crate::FieldReader<u8, HYSMODE_A>;
impl HYSMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSMODE_A {
        match self.bits {
            0 => HYSMODE_A::OFF,
            1 => HYSMODE_A::_10MV,
            2 => HYSMODE_A::_25MV,
            3 => HYSMODE_A::_50MV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == HYSMODE_A::OFF
    }
    #[doc = "Checks if the value of the field is `_10MV`"]
    #[inline(always)]
    pub fn is_10m_v(&self) -> bool {
        *self == HYSMODE_A::_10MV
    }
    #[doc = "Checks if the value of the field is `_25MV`"]
    #[inline(always)]
    pub fn is_25m_v(&self) -> bool {
        *self == HYSMODE_A::_25MV
    }
    #[doc = "Checks if the value of the field is `_50MV`"]
    #[inline(always)]
    pub fn is_50m_v(&self) -> bool {
        *self == HYSMODE_A::_50MV
    }
}
#[doc = "Field `HYSMODE` writer - Hysteresis Mode"]
pub type HYSMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CTRLA_SPEC, u8, HYSMODE_A, 2, O>;
impl<'a, const O: u8> HYSMODE_W<'a, O> {
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(HYSMODE_A::OFF)
    }
    #[doc = "10mV hysteresis"]
    #[inline(always)]
    pub fn _10m_v(self) -> &'a mut W {
        self.variant(HYSMODE_A::_10MV)
    }
    #[doc = "25mV hysteresis"]
    #[inline(always)]
    pub fn _25m_v(self) -> &'a mut W {
        self.variant(HYSMODE_A::_25MV)
    }
    #[doc = "50mV hysteresis"]
    #[inline(always)]
    pub fn _50m_v(self) -> &'a mut W {
        self.variant(HYSMODE_A::_50MV)
    }
}
#[doc = "Low Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPMODE_A {
    #[doc = "0: Low power mode disabled"]
    DIS = 0,
    #[doc = "1: Low power mode enabled"]
    EN = 1,
}
impl From<LPMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LPMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPMODE` reader - Low Power Mode"]
pub type LPMODE_R = crate::BitReader<LPMODE_A>;
impl LPMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMODE_A {
        match self.bits {
            false => LPMODE_A::DIS,
            true => LPMODE_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == LPMODE_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == LPMODE_A::EN
    }
}
#[doc = "Field `LPMODE` writer - Low Power Mode"]
pub type LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, LPMODE_A, O>;
impl<'a, const O: u8> LPMODE_W<'a, O> {
    #[doc = "Low power mode disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(LPMODE_A::DIS)
    }
    #[doc = "Low power mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LPMODE_A::EN)
    }
}
#[doc = "Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTMODE_A {
    #[doc = "0: Any Edge"]
    BOTHEDGE = 0,
    #[doc = "2: Negative Edge"]
    NEGEDGE = 2,
    #[doc = "3: Positive Edge"]
    POSEDGE = 3,
}
impl From<INTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INTMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTMODE` reader - Interrupt Mode"]
pub type INTMODE_R = crate::FieldReader<u8, INTMODE_A>;
impl INTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INTMODE_A> {
        match self.bits {
            0 => Some(INTMODE_A::BOTHEDGE),
            2 => Some(INTMODE_A::NEGEDGE),
            3 => Some(INTMODE_A::POSEDGE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BOTHEDGE`"]
    #[inline(always)]
    pub fn is_bothedge(&self) -> bool {
        *self == INTMODE_A::BOTHEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == INTMODE_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == INTMODE_A::POSEDGE
    }
}
#[doc = "Field `INTMODE` writer - Interrupt Mode"]
pub type INTMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CTRLA_SPEC, u8, INTMODE_A, 2, O>;
impl<'a, const O: u8> INTMODE_W<'a, O> {
    #[doc = "Any Edge"]
    #[inline(always)]
    pub fn bothedge(self) -> &'a mut W {
        self.variant(INTMODE_A::BOTHEDGE)
    }
    #[doc = "Negative Edge"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(INTMODE_A::NEGEDGE)
    }
    #[doc = "Positive Edge"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(INTMODE_A::POSEDGE)
    }
}
#[doc = "Field `OUTEN` reader - Output Buffer Enable"]
pub type OUTEN_R = crate::BitReader<bool>;
#[doc = "Field `OUTEN` writer - Output Buffer Enable"]
pub type OUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby Mode"]
pub type RUNSTDBY_R = crate::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run in Standby Mode"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    pub fn hysmode(&self) -> HYSMODE_R {
        HYSMODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Low Power Mode"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&self) -> INTMODE_R {
        INTMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output Buffer Enable"]
    #[inline(always)]
    pub fn outen(&self) -> OUTEN_R {
        OUTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:2 - Hysteresis Mode"]
    #[inline(always)]
    pub fn hysmode(&mut self) -> HYSMODE_W<1> {
        HYSMODE_W::new(self)
    }
    #[doc = "Bit 3 - Low Power Mode"]
    #[inline(always)]
    pub fn lpmode(&mut self) -> LPMODE_W<3> {
        LPMODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Interrupt Mode"]
    #[inline(always)]
    pub fn intmode(&mut self) -> INTMODE_W<4> {
        INTMODE_W::new(self)
    }
    #[doc = "Bit 6 - Output Buffer Enable"]
    #[inline(always)]
    pub fn outen(&mut self) -> OUTEN_W<6> {
        OUTEN_W::new(self)
    }
    #[doc = "Bit 7 - Run in Standby Mode"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<7> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
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
