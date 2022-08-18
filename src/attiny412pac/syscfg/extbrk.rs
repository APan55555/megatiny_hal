#[doc = "Register `EXTBRK` reader"]
pub struct R(crate::attiny412pac::R<EXTBRK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<EXTBRK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<EXTBRK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<EXTBRK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTBRK` writer"]
pub struct W(crate::attiny412pac::W<EXTBRK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<EXTBRK_SPEC>;
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
impl From<crate::attiny412pac::W<EXTBRK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<EXTBRK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENEXTBRK` reader - External break enable"]
pub type ENEXTBRK_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `ENEXTBRK` writer - External break enable"]
pub type ENEXTBRK_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, EXTBRK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - External break enable"]
    #[inline(always)]
    pub fn enextbrk(&self) -> ENEXTBRK_R {
        ENEXTBRK_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External break enable"]
    #[inline(always)]
    pub fn enextbrk(&mut self) -> ENEXTBRK_W<0> {
        ENEXTBRK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Break\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extbrk](index.html) module"]
pub struct EXTBRK_SPEC;
impl crate::attiny412pac::RegisterSpec for EXTBRK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extbrk::R](R) reader structure"]
impl crate::attiny412pac::Readable for EXTBRK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extbrk::W](W) writer structure"]
impl crate::attiny412pac::Writable for EXTBRK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTBRK to value 0"]
impl crate::attiny412pac::Resettable for EXTBRK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
