#[doc = "Register `STATUS` reader"]
pub struct R(crate::attiny412pac::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::attiny412pac::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<STATUS_SPEC>;
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
impl From<crate::attiny412pac::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVL0EX` reader - Level 0 Interrupt Executing"]
pub type LVL0EX_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `LVL1EX` reader - Level 1 Interrupt Executing"]
pub type LVL1EX_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `NMIEX` reader - Non-maskable Interrupt Executing"]
pub type NMIEX_R = crate::attiny412pac::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Level 0 Interrupt Executing"]
    #[inline(always)]
    pub fn lvl0ex(&self) -> LVL0EX_R {
        LVL0EX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1 Interrupt Executing"]
    #[inline(always)]
    pub fn lvl1ex(&self) -> LVL1EX_R {
        LVL1EX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Non-maskable Interrupt Executing"]
    #[inline(always)]
    pub fn nmiex(&self) -> NMIEX_R {
        NMIEX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::attiny412pac::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::attiny412pac::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::attiny412pac::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::attiny412pac::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
