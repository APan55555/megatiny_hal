#[doc = "Register `OSC20MCALIBB` reader"]
pub struct R(crate::attiny412pac::R<OSC20MCALIBB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<OSC20MCALIBB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<OSC20MCALIBB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<OSC20MCALIBB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC20MCALIBB` writer"]
pub struct W(crate::attiny412pac::W<OSC20MCALIBB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<OSC20MCALIBB_SPEC>;
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
impl From<crate::attiny412pac::W<OSC20MCALIBB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<OSC20MCALIBB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPCAL20M` reader - Oscillator temperature coefficient"]
pub type TEMPCAL20M_R = crate::attiny412pac::FieldReader<u8, u8>;
#[doc = "Field `TEMPCAL20M` writer - Oscillator temperature coefficient"]
pub type TEMPCAL20M_W<'a, const O: u8> =
    crate::attiny412pac::FieldWriterSafe<'a, u8, OSC20MCALIBB_SPEC, u8, u8, 4, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OSC20MCALIBB_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    pub fn tempcal20m(&self) -> TEMPCAL20M_R {
        TEMPCAL20M_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Oscillator temperature coefficient"]
    #[inline(always)]
    pub fn tempcal20m(&mut self) -> TEMPCAL20M_W<0> {
        TEMPCAL20M_W::new(self)
    }
    #[doc = "Bit 7 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W<7> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC20M Calibration B\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc20mcalibb](index.html) module"]
pub struct OSC20MCALIBB_SPEC;
impl crate::attiny412pac::RegisterSpec for OSC20MCALIBB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc20mcalibb::R](R) reader structure"]
impl crate::attiny412pac::Readable for OSC20MCALIBB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc20mcalibb::W](W) writer structure"]
impl crate::attiny412pac::Writable for OSC20MCALIBB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC20MCALIBB to value 0"]
impl crate::attiny412pac::Resettable for OSC20MCALIBB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
