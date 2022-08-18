#[doc = "Register `OSC32KCTRLA` reader"]
pub struct R(crate::attiny412pac::R<OSC32KCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<OSC32KCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<OSC32KCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<OSC32KCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC32KCTRLA` writer"]
pub struct W(crate::attiny412pac::W<OSC32KCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<OSC32KCTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<OSC32KCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<OSC32KCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OSC32KCTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Run standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W<1> {
        RUNSTDBY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC32K Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32kctrla](index.html) module"]
pub struct OSC32KCTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for OSC32KCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc32kctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for OSC32KCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc32kctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for OSC32KCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC32KCTRLA to value 0"]
impl crate::attiny412pac::Resettable for OSC32KCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
