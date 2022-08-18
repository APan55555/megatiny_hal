#[doc = "Register `CTRLB` reader"]
pub struct R(crate::attiny412pac::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::attiny412pac::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLB_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CRC Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: CRC on entire flash"]
    FLASH = 0,
    #[doc = "1: CRC on boot and appl section of flash"]
    APPLICATION = 1,
    #[doc = "2: CRC on boot section of flash"]
    BOOT = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRC` reader - CRC Source"]
pub type SRC_R = crate::attiny412pac::FieldReader<u8, SRC_A>;
impl SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_A> {
        match self.bits {
            0 => Some(SRC_A::FLASH),
            1 => Some(SRC_A::APPLICATION),
            2 => Some(SRC_A::BOOT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASH`"]
    #[inline(always)]
    pub fn is_flash(&self) -> bool {
        *self == SRC_A::FLASH
    }
    #[doc = "Checks if the value of the field is `APPLICATION`"]
    #[inline(always)]
    pub fn is_application(&self) -> bool {
        *self == SRC_A::APPLICATION
    }
    #[doc = "Checks if the value of the field is `BOOT`"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == SRC_A::BOOT
    }
}
#[doc = "Field `SRC` writer - CRC Source"]
pub type SRC_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, CTRLB_SPEC, u8, SRC_A, 2, O>;
impl<'a, const O: u8> SRC_W<'a, O> {
    #[doc = "CRC on entire flash"]
    #[inline(always)]
    pub fn flash(self) -> &'a mut W {
        self.variant(SRC_A::FLASH)
    }
    #[doc = "CRC on boot and appl section of flash"]
    #[inline(always)]
    pub fn application(self) -> &'a mut W {
        self.variant(SRC_A::APPLICATION)
    }
    #[doc = "CRC on boot section of flash"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut W {
        self.variant(SRC_A::BOOT)
    }
}
#[doc = "CRC Flash Access Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Priority to flash"]
    PRIORITY = 0,
    #[doc = "2: Lowest priority to flash"]
    BACKGROUND = 2,
    #[doc = "3: Continuous checks in background"]
    CONTINUOUS = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE` reader - CRC Flash Access Mode"]
pub type MODE_R = crate::attiny412pac::FieldReader<u8, MODE_A>;
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::PRIORITY,
            2 => MODE_A::BACKGROUND,
            3 => MODE_A::CONTINUOUS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRIORITY`"]
    #[inline(always)]
    pub fn is_priority(&self) -> bool {
        *self == MODE_A::PRIORITY
    }
    #[doc = "Checks if the value of the field is `BACKGROUND`"]
    #[inline(always)]
    pub fn is_background(&self) -> bool {
        *self == MODE_A::BACKGROUND
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == MODE_A::CONTINUOUS
    }
}
#[doc = "Field `MODE` writer - CRC Flash Access Mode"]
pub type MODE_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, CTRLB_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Priority to flash"]
    #[inline(always)]
    pub fn priority(self) -> &'a mut W {
        self.variant(MODE_A::PRIORITY)
    }
    #[doc = "Lowest priority to flash"]
    #[inline(always)]
    pub fn background(self) -> &'a mut W {
        self.variant(MODE_A::BACKGROUND)
    }
    #[doc = "Continuous checks in background"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(MODE_A::CONTINUOUS)
    }
}
impl R {
    #[doc = "Bits 0:1 - CRC Source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - CRC Flash Access Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC Source"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 4:5 - CRC Flash Access Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::attiny412pac::Resettable for CTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
