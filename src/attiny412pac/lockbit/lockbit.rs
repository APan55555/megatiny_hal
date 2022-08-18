#[doc = "Register `LOCKBIT` reader"]
pub struct R(crate::attiny412pac::R<LOCKBIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<LOCKBIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<LOCKBIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<LOCKBIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCKBIT` writer"]
pub struct W(crate::attiny412pac::W<LOCKBIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<LOCKBIT_SPEC>;
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
impl From<crate::attiny412pac::W<LOCKBIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<LOCKBIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LB_A {
    #[doc = "58: Read and write lock"]
    RWLOCK = 58,
    #[doc = "197: No locks"]
    NOLOCK = 197,
}
impl From<LB_A> for u8 {
    #[inline(always)]
    fn from(variant: LB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LB` reader - Lock Bits"]
pub type LB_R = crate::attiny412pac::FieldReader<u8, LB_A>;
impl LB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LB_A> {
        match self.bits {
            58 => Some(LB_A::RWLOCK),
            197 => Some(LB_A::NOLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RWLOCK`"]
    #[inline(always)]
    pub fn is_rwlock(&self) -> bool {
        *self == LB_A::RWLOCK
    }
    #[doc = "Checks if the value of the field is `NOLOCK`"]
    #[inline(always)]
    pub fn is_nolock(&self) -> bool {
        *self == LB_A::NOLOCK
    }
}
#[doc = "Field `LB` writer - Lock Bits"]
pub type LB_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, LOCKBIT_SPEC, u8, LB_A, 8, O>;
impl<'a, const O: u8> LB_W<'a, O> {
    #[doc = "Read and write lock"]
    #[inline(always)]
    pub fn rwlock(self) -> &'a mut W {
        self.variant(LB_A::RWLOCK)
    }
    #[doc = "No locks"]
    #[inline(always)]
    pub fn nolock(self) -> &'a mut W {
        self.variant(LB_A::NOLOCK)
    }
}
impl R {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    pub fn lb(&self) -> LB_R {
        LB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Lock Bits"]
    #[inline(always)]
    pub fn lb(&mut self) -> LB_W<0> {
        LB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock bits\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lockbit](index.html) module"]
pub struct LOCKBIT_SPEC;
impl crate::attiny412pac::RegisterSpec for LOCKBIT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lockbit::R](R) reader structure"]
impl crate::attiny412pac::Readable for LOCKBIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lockbit::W](W) writer structure"]
impl crate::attiny412pac::Writable for LOCKBIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LOCKBIT to value 0"]
impl crate::attiny412pac::Resettable for LOCKBIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
