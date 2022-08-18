#[doc = "Register `COMMAND` reader"]
pub struct R(crate::attiny412pac::R<COMMAND_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<COMMAND_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<COMMAND_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<COMMAND_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMMAND` writer"]
pub struct W(crate::attiny412pac::W<COMMAND_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<COMMAND_SPEC>;
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
impl From<crate::attiny412pac::W<COMMAND_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<COMMAND_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STCONV` reader - Start Conversion Operation"]
pub type STCONV_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `STCONV` writer - Start Conversion Operation"]
pub type STCONV_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, COMMAND_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start Conversion Operation"]
    #[inline(always)]
    pub fn stconv(&self) -> STCONV_R {
        STCONV_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Operation"]
    #[inline(always)]
    pub fn stconv(&mut self) -> STCONV_W<0> {
        STCONV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [command](index.html) module"]
pub struct COMMAND_SPEC;
impl crate::attiny412pac::RegisterSpec for COMMAND_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [command::R](R) reader structure"]
impl crate::attiny412pac::Readable for COMMAND_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [command::W](W) writer structure"]
impl crate::attiny412pac::Writable for COMMAND_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::attiny412pac::Resettable for COMMAND_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
