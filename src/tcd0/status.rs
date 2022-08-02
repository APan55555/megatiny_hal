#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENRDY` reader - Enable ready"]
pub type ENRDY_R = crate::BitReader<bool>;
#[doc = "Field `CMDRDY` reader - Command ready"]
pub type CMDRDY_R = crate::BitReader<bool>;
#[doc = "Field `PWMACTA` reader - PWM activity on A"]
pub type PWMACTA_R = crate::BitReader<bool>;
#[doc = "Field `PWMACTA` writer - PWM activity on A"]
pub type PWMACTA_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
#[doc = "Field `PWMACTB` reader - PWM activity on B"]
pub type PWMACTB_R = crate::BitReader<bool>;
#[doc = "Field `PWMACTB` writer - PWM activity on B"]
pub type PWMACTB_W<'a, const O: u8> = crate::BitWriter<'a, u8, STATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable ready"]
    #[inline(always)]
    pub fn enrdy(&self) -> ENRDY_R {
        ENRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command ready"]
    #[inline(always)]
    pub fn cmdrdy(&self) -> CMDRDY_R {
        CMDRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM activity on A"]
    #[inline(always)]
    pub fn pwmacta(&self) -> PWMACTA_R {
        PWMACTA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM activity on B"]
    #[inline(always)]
    pub fn pwmactb(&self) -> PWMACTB_R {
        PWMACTB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - PWM activity on A"]
    #[inline(always)]
    pub fn pwmacta(&mut self) -> PWMACTA_W<6> {
        PWMACTA_W::new(self)
    }
    #[doc = "Bit 7 - PWM activity on B"]
    #[inline(always)]
    pub fn pwmactb(&mut self) -> PWMACTB_W<7> {
        PWMACTB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
