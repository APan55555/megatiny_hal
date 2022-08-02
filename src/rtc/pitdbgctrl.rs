#[doc = "Register `PITDBGCTRL` reader"]
pub struct R(crate::R<PITDBGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PITDBGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PITDBGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PITDBGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PITDBGCTRL` writer"]
pub struct W(crate::W<PITDBGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PITDBGCTRL_SPEC>;
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
impl From<crate::W<PITDBGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PITDBGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGRUN` reader - Run in debug"]
pub type DBGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DBGRUN` writer - Run in debug"]
pub type DBGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, PITDBGCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Run in debug"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run in debug"]
    #[inline(always)]
    pub fn dbgrun(&mut self) -> DBGRUN_W<0> {
        DBGRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIT Debug control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitdbgctrl](index.html) module"]
pub struct PITDBGCTRL_SPEC;
impl crate::RegisterSpec for PITDBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitdbgctrl::R](R) reader structure"]
impl crate::Readable for PITDBGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pitdbgctrl::W](W) writer structure"]
impl crate::Writable for PITDBGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PITDBGCTRL to value 0"]
impl crate::Resettable for PITDBGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
