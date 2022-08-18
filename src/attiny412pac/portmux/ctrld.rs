#[doc = "Register `CTRLD` reader"]
pub struct R(crate::attiny412pac::R<CTRLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLD` writer"]
pub struct W(crate::attiny412pac::W<CTRLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLD_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Multiplexer TCB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCB0_A {
    #[doc = "0: Default pin"]
    DEFAULT = 0,
    #[doc = "1: Alternate pin"]
    ALTERNATE = 1,
}
impl From<TCB0_A> for bool {
    #[inline(always)]
    fn from(variant: TCB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCB0` reader - Port Multiplexer TCB"]
pub type TCB0_R = crate::attiny412pac::BitReader<TCB0_A>;
impl TCB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCB0_A {
        match self.bits {
            false => TCB0_A::DEFAULT,
            true => TCB0_A::ALTERNATE,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULT`"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == TCB0_A::DEFAULT
    }
    #[doc = "Checks if the value of the field is `ALTERNATE`"]
    #[inline(always)]
    pub fn is_alternate(&self) -> bool {
        *self == TCB0_A::ALTERNATE
    }
}
#[doc = "Field `TCB0` writer - Port Multiplexer TCB"]
pub type TCB0_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLD_SPEC, TCB0_A, O>;
impl<'a, const O: u8> TCB0_W<'a, O> {
    #[doc = "Default pin"]
    #[inline(always)]
    pub fn default(self) -> &'a mut W {
        self.variant(TCB0_A::DEFAULT)
    }
    #[doc = "Alternate pin"]
    #[inline(always)]
    pub fn alternate(self) -> &'a mut W {
        self.variant(TCB0_A::ALTERNATE)
    }
}
impl R {
    #[doc = "Bit 0 - Port Multiplexer TCB"]
    #[inline(always)]
    pub fn tcb0(&self) -> TCB0_R {
        TCB0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Multiplexer TCB"]
    #[inline(always)]
    pub fn tcb0(&mut self) -> TCB0_W<0> {
        TCB0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Multiplexer Control D\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrld](index.html) module"]
pub struct CTRLD_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrld::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrld::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLD to value 0"]
impl crate::attiny412pac::Resettable for CTRLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
