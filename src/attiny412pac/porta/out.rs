#[doc = "Register `OUT` reader"]
pub struct R(crate::attiny412pac::R<OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT` writer"]
pub struct W(crate::attiny412pac::W<OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<OUT_SPEC>;
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
impl From<crate::attiny412pac::W<OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PA0` reader - Pin A0"]
pub type PA0_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA0` writer - Pin A0"]
pub type PA0_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PA1` reader - Pin A1"]
pub type PA1_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA1` writer - Pin A1"]
pub type PA1_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PA2` reader - Pin A2"]
pub type PA2_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA2` writer - Pin A2"]
pub type PA2_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PA3` reader - Pin A3"]
pub type PA3_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA3` writer - Pin A3"]
pub type PA3_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PA6` reader - Pin A6"]
pub type PA6_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA6` writer - Pin A6"]
pub type PA6_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
#[doc = "Field `PA7` reader - Pin A7"]
pub type PA7_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PA7` writer - Pin A7"]
pub type PA7_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, OUT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    pub fn pa0(&self) -> PA0_R {
        PA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    pub fn pa1(&self) -> PA1_R {
        PA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    pub fn pa2(&self) -> PA2_R {
        PA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    pub fn pa3(&self) -> PA3_R {
        PA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    pub fn pa6(&self) -> PA6_R {
        PA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    pub fn pa7(&self) -> PA7_R {
        PA7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pin A0"]
    #[inline(always)]
    pub fn pa0(&mut self) -> PA0_W<0> {
        PA0_W::new(self)
    }
    #[doc = "Bit 1 - Pin A1"]
    #[inline(always)]
    pub fn pa1(&mut self) -> PA1_W<1> {
        PA1_W::new(self)
    }
    #[doc = "Bit 2 - Pin A2"]
    #[inline(always)]
    pub fn pa2(&mut self) -> PA2_W<2> {
        PA2_W::new(self)
    }
    #[doc = "Bit 3 - Pin A3"]
    #[inline(always)]
    pub fn pa3(&mut self) -> PA3_W<3> {
        PA3_W::new(self)
    }
    #[doc = "Bit 6 - Pin A6"]
    #[inline(always)]
    pub fn pa6(&mut self) -> PA6_W<6> {
        PA6_W::new(self)
    }
    #[doc = "Bit 7 - Pin A7"]
    #[inline(always)]
    pub fn pa7(&mut self) -> PA7_W<7> {
        PA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Value\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out](index.html) module"]
pub struct OUT_SPEC;
impl crate::attiny412pac::RegisterSpec for OUT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [out::R](R) reader structure"]
impl crate::attiny412pac::Readable for OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out::W](W) writer structure"]
impl crate::attiny412pac::Writable for OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT to value 0"]
impl crate::attiny412pac::Resettable for OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
