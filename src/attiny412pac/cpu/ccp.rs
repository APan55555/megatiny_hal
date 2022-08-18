#[doc = "Register `CCP` reader"]
pub struct R(crate::attiny412pac::R<CCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CCP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCP` writer"]
pub struct W(crate::attiny412pac::W<CCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CCP_SPEC>;
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
impl From<crate::attiny412pac::W<CCP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CCP signature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCP_A {
    #[doc = "157: SPM Instruction Protection"]
    SPM = 157,
    #[doc = "216: IO Register Protection"]
    IOREG = 216,
}
impl From<CCP_A> for u8 {
    #[inline(always)]
    fn from(variant: CCP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CCP` reader - CCP signature"]
pub type CCP_R = crate::attiny412pac::FieldReader<u8, CCP_A>;
impl CCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CCP_A> {
        match self.bits {
            157 => Some(CCP_A::SPM),
            216 => Some(CCP_A::IOREG),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPM`"]
    #[inline(always)]
    pub fn is_spm(&self) -> bool {
        *self == CCP_A::SPM
    }
    #[doc = "Checks if the value of the field is `IOREG`"]
    #[inline(always)]
    pub fn is_ioreg(&self) -> bool {
        *self == CCP_A::IOREG
    }
}
#[doc = "Field `CCP` writer - CCP signature"]
pub type CCP_W<'a, const O: u8> = crate::attiny412pac::FieldWriter<'a, u8, CCP_SPEC, u8, CCP_A, 8, O>;
impl<'a, const O: u8> CCP_W<'a, O> {
    #[doc = "SPM Instruction Protection"]
    #[inline(always)]
    pub fn spm(self) -> &'a mut W {
        self.variant(CCP_A::SPM)
    }
    #[doc = "IO Register Protection"]
    #[inline(always)]
    pub fn ioreg(self) -> &'a mut W {
        self.variant(CCP_A::IOREG)
    }
}
impl R {
    #[doc = "Bits 0:7 - CCP signature"]
    #[inline(always)]
    pub fn ccp(&self) -> CCP_R {
        CCP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CCP signature"]
    #[inline(always)]
    pub fn ccp(&mut self) -> CCP_W<0> {
        CCP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Change Protection\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccp](index.html) module"]
pub struct CCP_SPEC;
impl crate::attiny412pac::RegisterSpec for CCP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ccp::R](R) reader structure"]
impl crate::attiny412pac::Readable for CCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccp::W](W) writer structure"]
impl crate::attiny412pac::Writable for CCP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCP to value 0"]
impl crate::attiny412pac::Resettable for CCP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
