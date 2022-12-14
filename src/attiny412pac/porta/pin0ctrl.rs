#[doc = "Register `PIN0CTRL` reader"]
pub struct R(crate::attiny412pac::R<PIN0CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<PIN0CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<PIN0CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<PIN0CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN0CTRL` writer"]
pub struct W(crate::attiny412pac::W<PIN0CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<PIN0CTRL_SPEC>;
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
impl From<crate::attiny412pac::W<PIN0CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<PIN0CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input/Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ISC_A {
    #[doc = "0: Iterrupt disabled but input buffer enabled"]
    INTDISABLE = 0,
    #[doc = "1: Sense Both Edges"]
    BOTHEDGES = 1,
    #[doc = "2: Sense Rising Edge"]
    RISING = 2,
    #[doc = "3: Sense Falling Edge"]
    FALLING = 3,
    #[doc = "4: Digital Input Buffer disabled"]
    INPUT_DISABLE = 4,
    #[doc = "5: Sense low Level"]
    LEVEL = 5,
}
impl From<ISC_A> for u8 {
    #[inline(always)]
    fn from(variant: ISC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ISC` reader - Input/Sense Configuration"]
pub type ISC_R = crate::attiny412pac::FieldReader<u8, ISC_A>;
impl ISC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ISC_A> {
        match self.bits {
            0 => Some(ISC_A::INTDISABLE),
            1 => Some(ISC_A::BOTHEDGES),
            2 => Some(ISC_A::RISING),
            3 => Some(ISC_A::FALLING),
            4 => Some(ISC_A::INPUT_DISABLE),
            5 => Some(ISC_A::LEVEL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTDISABLE`"]
    #[inline(always)]
    pub fn is_intdisable(&self) -> bool {
        *self == ISC_A::INTDISABLE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == ISC_A::BOTHEDGES
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == ISC_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == ISC_A::FALLING
    }
    #[doc = "Checks if the value of the field is `INPUT_DISABLE`"]
    #[inline(always)]
    pub fn is_input_disable(&self) -> bool {
        *self == ISC_A::INPUT_DISABLE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == ISC_A::LEVEL
    }
}
#[doc = "Field `ISC` writer - Input/Sense Configuration"]
pub type ISC_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, PIN0CTRL_SPEC, u8, ISC_A, 3, O>;
impl<'a, const O: u8> ISC_W<'a, O> {
    #[doc = "Iterrupt disabled but input buffer enabled"]
    #[inline(always)]
    pub fn intdisable(self) -> &'a mut W {
        self.variant(ISC_A::INTDISABLE)
    }
    #[doc = "Sense Both Edges"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(ISC_A::BOTHEDGES)
    }
    #[doc = "Sense Rising Edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(ISC_A::RISING)
    }
    #[doc = "Sense Falling Edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(ISC_A::FALLING)
    }
    #[doc = "Digital Input Buffer disabled"]
    #[inline(always)]
    pub fn input_disable(self) -> &'a mut W {
        self.variant(ISC_A::INPUT_DISABLE)
    }
    #[doc = "Sense low Level"]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(ISC_A::LEVEL)
    }
}
#[doc = "Field `PULLUPEN` reader - Pullup enable"]
pub type PULLUPEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PULLUPEN` writer - Pullup enable"]
pub type PULLUPEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, PIN0CTRL_SPEC, bool, O>;
#[doc = "Field `INVEN` reader - Inverted I/O Enable"]
pub type INVEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `INVEN` writer - Inverted I/O Enable"]
pub type INVEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, PIN0CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Input/Sense Configuration"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Pullup enable"]
    #[inline(always)]
    pub fn pullupen(&self) -> PULLUPEN_R {
        PULLUPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Inverted I/O Enable"]
    #[inline(always)]
    pub fn inven(&self) -> INVEN_R {
        INVEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input/Sense Configuration"]
    #[inline(always)]
    pub fn isc(&mut self) -> ISC_W<0> {
        ISC_W::new(self)
    }
    #[doc = "Bit 3 - Pullup enable"]
    #[inline(always)]
    pub fn pullupen(&mut self) -> PULLUPEN_W<3> {
        PULLUPEN_W::new(self)
    }
    #[doc = "Bit 7 - Inverted I/O Enable"]
    #[inline(always)]
    pub fn inven(&mut self) -> INVEN_W<7> {
        INVEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin 0 Control\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin0ctrl](index.html) module"]
pub struct PIN0CTRL_SPEC;
impl crate::attiny412pac::RegisterSpec for PIN0CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pin0ctrl::R](R) reader structure"]
impl crate::attiny412pac::Readable for PIN0CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin0ctrl::W](W) writer structure"]
impl crate::attiny412pac::Writable for PIN0CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PIN0CTRL to value 0"]
impl crate::attiny412pac::Resettable for PIN0CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
