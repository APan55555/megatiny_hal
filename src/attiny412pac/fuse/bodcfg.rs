#[doc = "Register `BODCFG` reader"]
pub struct R(crate::attiny412pac::R<BODCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<BODCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<BODCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<BODCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BODCFG` writer"]
pub struct W(crate::attiny412pac::W<BODCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<BODCFG_SPEC>;
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
impl From<crate::attiny412pac::W<BODCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<BODCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "BOD Operation in Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SLEEP_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
}
impl From<SLEEP_A> for u8 {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SLEEP` reader - BOD Operation in Sleep Mode"]
pub type SLEEP_R = crate::attiny412pac::FieldReader<u8, SLEEP_A>;
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEEP_A> {
        match self.bits {
            0 => Some(SLEEP_A::DIS),
            1 => Some(SLEEP_A::ENABLED),
            2 => Some(SLEEP_A::SAMPLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == SLEEP_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLEEP_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == SLEEP_A::SAMPLED
    }
}
#[doc = "Field `SLEEP` writer - BOD Operation in Sleep Mode"]
pub type SLEEP_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, BODCFG_SPEC, u8, SLEEP_A, 2, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SLEEP_A::DIS)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLEEP_A::ENABLED)
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut W {
        self.variant(SLEEP_A::SAMPLED)
    }
}
#[doc = "BOD Operation in Active Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACTIVE_A {
    #[doc = "0: Disabled"]
    DIS = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
    #[doc = "2: Sampled"]
    SAMPLED = 2,
    #[doc = "3: Enabled with wake-up halted until BOD is ready"]
    ENWAKE = 3,
}
impl From<ACTIVE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACTIVE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACTIVE` reader - BOD Operation in Active Mode"]
pub type ACTIVE_R = crate::attiny412pac::FieldReader<u8, ACTIVE_A>;
impl ACTIVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACTIVE_A {
        match self.bits {
            0 => ACTIVE_A::DIS,
            1 => ACTIVE_A::ENABLED,
            2 => ACTIVE_A::SAMPLED,
            3 => ACTIVE_A::ENWAKE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACTIVE_A::DIS
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACTIVE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `SAMPLED`"]
    #[inline(always)]
    pub fn is_sampled(&self) -> bool {
        *self == ACTIVE_A::SAMPLED
    }
    #[doc = "Checks if the value of the field is `ENWAKE`"]
    #[inline(always)]
    pub fn is_enwake(&self) -> bool {
        *self == ACTIVE_A::ENWAKE
    }
}
#[doc = "Field `ACTIVE` writer - BOD Operation in Active Mode"]
pub type ACTIVE_W<'a, const O: u8> =
    crate::attiny412pac::FieldWriterSafe<'a, u8, BODCFG_SPEC, u8, ACTIVE_A, 2, O>;
impl<'a, const O: u8> ACTIVE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACTIVE_A::DIS)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENABLED)
    }
    #[doc = "Sampled"]
    #[inline(always)]
    pub fn sampled(self) -> &'a mut W {
        self.variant(ACTIVE_A::SAMPLED)
    }
    #[doc = "Enabled with wake-up halted until BOD is ready"]
    #[inline(always)]
    pub fn enwake(self) -> &'a mut W {
        self.variant(ACTIVE_A::ENWAKE)
    }
}
#[doc = "BOD Sample Frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPFREQ_A {
    #[doc = "0: 1kHz sampling frequency"]
    _1KHZ = 0,
    #[doc = "1: 125Hz sampling frequency"]
    _125HZ = 1,
}
impl From<SAMPFREQ_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPFREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAMPFREQ` reader - BOD Sample Frequency"]
pub type SAMPFREQ_R = crate::attiny412pac::BitReader<SAMPFREQ_A>;
impl SAMPFREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPFREQ_A {
        match self.bits {
            false => SAMPFREQ_A::_1KHZ,
            true => SAMPFREQ_A::_125HZ,
        }
    }
    #[doc = "Checks if the value of the field is `_1KHZ`"]
    #[inline(always)]
    pub fn is_1khz(&self) -> bool {
        *self == SAMPFREQ_A::_1KHZ
    }
    #[doc = "Checks if the value of the field is `_125HZ`"]
    #[inline(always)]
    pub fn is_125hz(&self) -> bool {
        *self == SAMPFREQ_A::_125HZ
    }
}
#[doc = "Field `SAMPFREQ` writer - BOD Sample Frequency"]
pub type SAMPFREQ_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, BODCFG_SPEC, SAMPFREQ_A, O>;
impl<'a, const O: u8> SAMPFREQ_W<'a, O> {
    #[doc = "1kHz sampling frequency"]
    #[inline(always)]
    pub fn _1khz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_1KHZ)
    }
    #[doc = "125Hz sampling frequency"]
    #[inline(always)]
    pub fn _125hz(self) -> &'a mut W {
        self.variant(SAMPFREQ_A::_125HZ)
    }
}
#[doc = "BOD Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVL_A {
    #[doc = "0: 1.8 V"]
    BODLEVEL0 = 0,
    #[doc = "1: 2.1 V"]
    BODLEVEL1 = 1,
    #[doc = "2: 2.6 V"]
    BODLEVEL2 = 2,
    #[doc = "3: 2.9 V"]
    BODLEVEL3 = 3,
    #[doc = "4: 3.3 V"]
    BODLEVEL4 = 4,
    #[doc = "5: 3.7 V"]
    BODLEVEL5 = 5,
    #[doc = "6: 4.0 V"]
    BODLEVEL6 = 6,
    #[doc = "7: 4.2 V"]
    BODLEVEL7 = 7,
}
impl From<LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVL` reader - BOD Level"]
pub type LVL_R = crate::attiny412pac::FieldReader<u8, LVL_A>;
impl LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVL_A {
        match self.bits {
            0 => LVL_A::BODLEVEL0,
            1 => LVL_A::BODLEVEL1,
            2 => LVL_A::BODLEVEL2,
            3 => LVL_A::BODLEVEL3,
            4 => LVL_A::BODLEVEL4,
            5 => LVL_A::BODLEVEL5,
            6 => LVL_A::BODLEVEL6,
            7 => LVL_A::BODLEVEL7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BODLEVEL0`"]
    #[inline(always)]
    pub fn is_bodlevel0(&self) -> bool {
        *self == LVL_A::BODLEVEL0
    }
    #[doc = "Checks if the value of the field is `BODLEVEL1`"]
    #[inline(always)]
    pub fn is_bodlevel1(&self) -> bool {
        *self == LVL_A::BODLEVEL1
    }
    #[doc = "Checks if the value of the field is `BODLEVEL2`"]
    #[inline(always)]
    pub fn is_bodlevel2(&self) -> bool {
        *self == LVL_A::BODLEVEL2
    }
    #[doc = "Checks if the value of the field is `BODLEVEL3`"]
    #[inline(always)]
    pub fn is_bodlevel3(&self) -> bool {
        *self == LVL_A::BODLEVEL3
    }
    #[doc = "Checks if the value of the field is `BODLEVEL4`"]
    #[inline(always)]
    pub fn is_bodlevel4(&self) -> bool {
        *self == LVL_A::BODLEVEL4
    }
    #[doc = "Checks if the value of the field is `BODLEVEL5`"]
    #[inline(always)]
    pub fn is_bodlevel5(&self) -> bool {
        *self == LVL_A::BODLEVEL5
    }
    #[doc = "Checks if the value of the field is `BODLEVEL6`"]
    #[inline(always)]
    pub fn is_bodlevel6(&self) -> bool {
        *self == LVL_A::BODLEVEL6
    }
    #[doc = "Checks if the value of the field is `BODLEVEL7`"]
    #[inline(always)]
    pub fn is_bodlevel7(&self) -> bool {
        *self == LVL_A::BODLEVEL7
    }
}
#[doc = "Field `LVL` writer - BOD Level"]
pub type LVL_W<'a, const O: u8> = crate::attiny412pac::FieldWriterSafe<'a, u8, BODCFG_SPEC, u8, LVL_A, 3, O>;
impl<'a, const O: u8> LVL_W<'a, O> {
    #[doc = "1.8 V"]
    #[inline(always)]
    pub fn bodlevel0(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL0)
    }
    #[doc = "2.1 V"]
    #[inline(always)]
    pub fn bodlevel1(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL1)
    }
    #[doc = "2.6 V"]
    #[inline(always)]
    pub fn bodlevel2(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL2)
    }
    #[doc = "2.9 V"]
    #[inline(always)]
    pub fn bodlevel3(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL3)
    }
    #[doc = "3.3 V"]
    #[inline(always)]
    pub fn bodlevel4(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL4)
    }
    #[doc = "3.7 V"]
    #[inline(always)]
    pub fn bodlevel5(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL5)
    }
    #[doc = "4.0 V"]
    #[inline(always)]
    pub fn bodlevel6(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL6)
    }
    #[doc = "4.2 V"]
    #[inline(always)]
    pub fn bodlevel7(self) -> &'a mut W {
        self.variant(LVL_A::BODLEVEL7)
    }
}
impl R {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&self) -> SAMPFREQ_R {
        SAMPFREQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&self) -> LVL_R {
        LVL_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BOD Operation in Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<0> {
        SLEEP_W::new(self)
    }
    #[doc = "Bits 2:3 - BOD Operation in Active Mode"]
    #[inline(always)]
    pub fn active(&mut self) -> ACTIVE_W<2> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 4 - BOD Sample Frequency"]
    #[inline(always)]
    pub fn sampfreq(&mut self) -> SAMPFREQ_W<4> {
        SAMPFREQ_W::new(self)
    }
    #[doc = "Bits 5:7 - BOD Level"]
    #[inline(always)]
    pub fn lvl(&mut self) -> LVL_W<5> {
        LVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD Configuration\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bodcfg](index.html) module"]
pub struct BODCFG_SPEC;
impl crate::attiny412pac::RegisterSpec for BODCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bodcfg::R](R) reader structure"]
impl crate::attiny412pac::Readable for BODCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bodcfg::W](W) writer structure"]
impl crate::attiny412pac::Writable for BODCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BODCFG to value 0"]
impl crate::attiny412pac::Resettable for BODCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
