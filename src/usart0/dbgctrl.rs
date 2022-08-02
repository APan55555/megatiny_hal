#[doc = "Register `DBGCTRL` reader"]
pub struct R(crate::R<DBGCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGCTRL` writer"]
pub struct W(crate::W<DBGCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCTRL_SPEC>;
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
impl From<crate::W<DBGCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DBGRUN` reader - Debug Run"]
pub type DBGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DBGRUN` writer - Debug Run"]
pub type DBGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u8, DBGCTRL_SPEC, bool, O>;
#[doc = "Field `ABMBP` reader - Autobaud majority voter bypass"]
pub type ABMBP_R = crate::BitReader<bool>;
#[doc = "Field `ABMBP` writer - Autobaud majority voter bypass"]
pub type ABMBP_W<'a, const O: u8> = crate::BitWriter<'a, u8, DBGCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Autobaud majority voter bypass"]
    #[inline(always)]
    pub fn abmbp(&self) -> ABMBP_R {
        ABMBP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run"]
    #[inline(always)]
    pub fn dbgrun(&mut self) -> DBGRUN_W<0> {
        DBGRUN_W::new(self)
    }
    #[doc = "Bit 7 - Autobaud majority voter bypass"]
    #[inline(always)]
    pub fn abmbp(&mut self) -> ABMBP_W<7> {
        ABMBP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgctrl](index.html) module"]
pub struct DBGCTRL_SPEC;
impl crate::RegisterSpec for DBGCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dbgctrl::R](R) reader structure"]
impl crate::Readable for DBGCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgctrl::W](W) writer structure"]
impl crate::Writable for DBGCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DBGCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
