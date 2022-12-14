#[doc = "Register `ASYNCUSER5` reader"]
pub struct R(crate::attiny412pac::R<ASYNCUSER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<ASYNCUSER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<ASYNCUSER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<ASYNCUSER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASYNCUSER5` writer"]
pub struct W(crate::attiny412pac::W<ASYNCUSER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<ASYNCUSER5_SPEC>;
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
impl From<crate::attiny412pac::W<ASYNCUSER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<ASYNCUSER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASYNCUSER5_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Synchronous Event Channel 0"]
    SYNCCH0 = 1,
    #[doc = "2: Synchronous Event Channel 1"]
    SYNCCH1 = 2,
    #[doc = "3: Asynchronous Event Channel 0"]
    ASYNCCH0 = 3,
    #[doc = "4: Asynchronous Event Channel 1"]
    ASYNCCH1 = 4,
    #[doc = "5: Asynchronous Event Channel 2"]
    ASYNCCH2 = 5,
    #[doc = "6: Asynchronous Event Channel 3"]
    ASYNCCH3 = 6,
}
impl From<ASYNCUSER5_A> for u8 {
    #[inline(always)]
    fn from(variant: ASYNCUSER5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ASYNCUSER5` reader - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub type ASYNCUSER5_R = crate::attiny412pac::FieldReader<u8, ASYNCUSER5_A>;
impl ASYNCUSER5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ASYNCUSER5_A> {
        match self.bits {
            0 => Some(ASYNCUSER5_A::OFF),
            1 => Some(ASYNCUSER5_A::SYNCCH0),
            2 => Some(ASYNCUSER5_A::SYNCCH1),
            3 => Some(ASYNCUSER5_A::ASYNCCH0),
            4 => Some(ASYNCUSER5_A::ASYNCCH1),
            5 => Some(ASYNCUSER5_A::ASYNCCH2),
            6 => Some(ASYNCUSER5_A::ASYNCCH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ASYNCUSER5_A::OFF
    }
    #[doc = "Checks if the value of the field is `SYNCCH0`"]
    #[inline(always)]
    pub fn is_syncch0(&self) -> bool {
        *self == ASYNCUSER5_A::SYNCCH0
    }
    #[doc = "Checks if the value of the field is `SYNCCH1`"]
    #[inline(always)]
    pub fn is_syncch1(&self) -> bool {
        *self == ASYNCUSER5_A::SYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH0`"]
    #[inline(always)]
    pub fn is_asyncch0(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH0
    }
    #[doc = "Checks if the value of the field is `ASYNCCH1`"]
    #[inline(always)]
    pub fn is_asyncch1(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH1
    }
    #[doc = "Checks if the value of the field is `ASYNCCH2`"]
    #[inline(always)]
    pub fn is_asyncch2(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH2
    }
    #[doc = "Checks if the value of the field is `ASYNCCH3`"]
    #[inline(always)]
    pub fn is_asyncch3(&self) -> bool {
        *self == ASYNCUSER5_A::ASYNCCH3
    }
}
#[doc = "Field `ASYNCUSER5` writer - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
pub type ASYNCUSER5_W<'a, const O: u8> =
    crate::attiny412pac::FieldWriter<'a, u8, ASYNCUSER5_SPEC, u8, ASYNCUSER5_A, 8, O>;
impl<'a, const O: u8> ASYNCUSER5_W<'a, O> {
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::OFF)
    }
    #[doc = "Synchronous Event Channel 0"]
    #[inline(always)]
    pub fn syncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::SYNCCH0)
    }
    #[doc = "Synchronous Event Channel 1"]
    #[inline(always)]
    pub fn syncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::SYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 0"]
    #[inline(always)]
    pub fn asyncch0(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::ASYNCCH0)
    }
    #[doc = "Asynchronous Event Channel 1"]
    #[inline(always)]
    pub fn asyncch1(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::ASYNCCH1)
    }
    #[doc = "Asynchronous Event Channel 2"]
    #[inline(always)]
    pub fn asyncch2(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::ASYNCCH2)
    }
    #[doc = "Asynchronous Event Channel 3"]
    #[inline(always)]
    pub fn asyncch3(self) -> &'a mut W {
        self.variant(ASYNCUSER5_A::ASYNCCH3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    #[inline(always)]
    pub fn asyncuser5(&self) -> ASYNCUSER5_R {
        ASYNCUSER5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1"]
    #[inline(always)]
    pub fn asyncuser5(&mut self) -> ASYNCUSER5_W<0> {
        ASYNCUSER5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Asynchronous User Ch 5 Input Selection - CCL LUT1 Event 1\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asyncuser5](index.html) module"]
pub struct ASYNCUSER5_SPEC;
impl crate::attiny412pac::RegisterSpec for ASYNCUSER5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [asyncuser5::R](R) reader structure"]
impl crate::attiny412pac::Readable for ASYNCUSER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asyncuser5::W](W) writer structure"]
impl crate::attiny412pac::Writable for ASYNCUSER5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASYNCUSER5 to value 0"]
impl crate::attiny412pac::Resettable for ASYNCUSER5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
