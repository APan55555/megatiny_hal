#[doc = "Register `OSC20MCTRLA` reader"]
pub struct R(crate::attiny412pac::R<OSC20MCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<OSC20MCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<OSC20MCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<OSC20MCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC20MCTRLA` writer"]
pub struct W(crate::attiny412pac::W<OSC20MCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<OSC20MCTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<OSC20MCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<OSC20MCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run standby"]
pub type RUNSTDBY_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RUNSTDBY` writer - Run standby"]
pub type RUNSTDBY_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OSC20MCTRLA_SPEC, bool, O>;
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
#[doc = "OSC20M Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc20mctrla](index.html) module"]
pub struct OSC20MCTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for OSC20MCTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc20mctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for OSC20MCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc20mctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for OSC20MCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC20MCTRLA to value 0"]
impl crate::attiny412pac::Resettable for OSC20MCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
