#[doc = "Register `TXDATAH` reader"]
pub struct R(crate::attiny412pac::R<TXDATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<TXDATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<TXDATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<TXDATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXDATAH` writer"]
pub struct W(crate::attiny412pac::W<TXDATAH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<TXDATAH_SPEC>;
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
impl From<crate::attiny412pac::W<TXDATAH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<TXDATAH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA8` reader - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `DATA8` writer - Transmit Data Register (CHSIZE=9bit)"]
pub type DATA8_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TXDATAH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Register (CHSIZE=9bit)"]
    #[inline(always)]
    pub fn data8(&mut self) -> DATA8_W<0> {
        DATA8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Data High Byte\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdatah](index.html) module"]
pub struct TXDATAH_SPEC;
impl crate::attiny412pac::RegisterSpec for TXDATAH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [txdatah::R](R) reader structure"]
impl crate::attiny412pac::Readable for TXDATAH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txdatah::W](W) writer structure"]
impl crate::attiny412pac::Writable for TXDATAH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXDATAH to value 0"]
impl crate::attiny412pac::Resettable for TXDATAH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
