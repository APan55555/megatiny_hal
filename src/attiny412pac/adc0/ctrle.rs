#[doc = "Register `CTRLE` reader"]
pub struct R(crate::attiny412pac::R<CTRLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLE` writer"]
pub struct W(crate::attiny412pac::W<CTRLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLE_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Window Comparator Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WINCM_A {
    #[doc = "0: No Window Comparison"]
    NONE = 0,
    #[doc = "1: Below Window"]
    BELOW = 1,
    #[doc = "2: Above Window"]
    ABOVE = 2,
    #[doc = "3: Inside Window"]
    INSIDE = 3,
    #[doc = "4: Outside Window"]
    OUTSIDE = 4,
}
impl From<WINCM_A> for u8 {
    #[inline(always)]
    fn from(variant: WINCM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WINCM` reader - Window Comparator Mode"]
pub type WINCM_R = crate::attiny412pac::FieldReader<u8, WINCM_A>;
impl WINCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WINCM_A> {
        match self.bits {
            0 => Some(WINCM_A::NONE),
            1 => Some(WINCM_A::BELOW),
            2 => Some(WINCM_A::ABOVE),
            3 => Some(WINCM_A::INSIDE),
            4 => Some(WINCM_A::OUTSIDE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == WINCM_A::NONE
    }
    #[doc = "Checks if the value of the field is `BELOW`"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == WINCM_A::BELOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == WINCM_A::ABOVE
    }
    #[doc = "Checks if the value of the field is `INSIDE`"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == WINCM_A::INSIDE
    }
    #[doc = "Checks if the value of the field is `OUTSIDE`"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == WINCM_A::OUTSIDE
    }
}
#[doc = "Field `WINCM` writer - Window Comparator Mode"]
pub type WINCM_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, CTRLE_SPEC, u8, WINCM_A, 3, O>;
impl<'a, const O: u8> WINCM_W<'a, O> {
    #[doc = "No Window Comparison"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(WINCM_A::NONE)
    }
    #[doc = "Below Window"]
    #[inline(always)]
    pub fn below(self) -> &'a mut W {
        self.variant(WINCM_A::BELOW)
    }
    #[doc = "Above Window"]
    #[inline(always)]
    pub fn above(self) -> &'a mut W {
        self.variant(WINCM_A::ABOVE)
    }
    #[doc = "Inside Window"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut W {
        self.variant(WINCM_A::INSIDE)
    }
    #[doc = "Outside Window"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut W {
        self.variant(WINCM_A::OUTSIDE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&self) -> WINCM_R {
        WINCM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Comparator Mode"]
    #[inline(always)]
    pub fn wincm(&mut self) -> WINCM_W<0> {
        WINCM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control E\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrle](index.html) module"]
pub struct CTRLE_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrle::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrle::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::attiny412pac::Resettable for CTRLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
