#[doc = "Register `CTRLA` reader"]
pub struct R(crate::attiny412pac::R<CTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<CTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<CTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<CTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLA` writer"]
pub struct W(crate::attiny412pac::W<CTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<CTRLA_SPEC>;
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
impl From<crate::attiny412pac::W<CTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<CTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Enable CRC scan"]
pub type ENABLE_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enable CRC scan"]
pub type ENABLE_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `NMIEN` reader - Enable NMI Trigger"]
pub type NMIEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `NMIEN` writer - Enable NMI Trigger"]
pub type NMIEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Reset CRC scan"]
pub type RESET_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RESET` writer - Reset CRC scan"]
pub type RESET_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, CTRLA_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable CRC scan"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable NMI Trigger"]
    #[inline(always)]
    pub fn nmien(&self) -> NMIEN_R {
        NMIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset CRC scan"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRC scan"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable NMI Trigger"]
    #[inline(always)]
    pub fn nmien(&mut self) -> NMIEN_W<1> {
        NMIEN_W::new(self)
    }
    #[doc = "Bit 7 - Reset CRC scan"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W<7> {
        RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control A\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrla](index.html) module"]
pub struct CTRLA_SPEC;
impl crate::attiny412pac::RegisterSpec for CTRLA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrla::R](R) reader structure"]
impl crate::attiny412pac::Readable for CTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrla::W](W) writer structure"]
impl crate::attiny412pac::Writable for CTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::attiny412pac::Resettable for CTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
