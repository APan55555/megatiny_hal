#[doc = "Register `TCD0CFG` reader"]
pub struct R(crate::attiny412pac::R<TCD0CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<TCD0CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<TCD0CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<TCD0CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD0CFG` writer"]
pub struct W(crate::attiny412pac::W<TCD0CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<TCD0CFG_SPEC>;
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
impl From<crate::attiny412pac::W<TCD0CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<TCD0CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPA` reader - Compare A Default Output Value"]
pub type CMPA_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPA` writer - Compare A Default Output Value"]
pub type CMPA_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPB` reader - Compare B Default Output Value"]
pub type CMPB_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPB` writer - Compare B Default Output Value"]
pub type CMPB_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPC` reader - Compare C Default Output Value"]
pub type CMPC_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPC` writer - Compare C Default Output Value"]
pub type CMPC_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPD` reader - Compare D Default Output Value"]
pub type CMPD_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPD` writer - Compare D Default Output Value"]
pub type CMPD_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPAEN` reader - Compare A Output Enable"]
pub type CMPAEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPAEN` writer - Compare A Output Enable"]
pub type CMPAEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPBEN` reader - Compare B Output Enable"]
pub type CMPBEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPBEN` writer - Compare B Output Enable"]
pub type CMPBEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPCEN` reader - Compare C Output Enable"]
pub type CMPCEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPCEN` writer - Compare C Output Enable"]
pub type CMPCEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
#[doc = "Field `CMPDEN` reader - Compare D Output Enable"]
pub type CMPDEN_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CMPDEN` writer - Compare D Output Enable"]
pub type CMPDEN_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, TCD0CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Compare A Default Output Value"]
    #[inline(always)]
    pub fn cmpa(&self) -> CMPA_R {
        CMPA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare B Default Output Value"]
    #[inline(always)]
    pub fn cmpb(&self) -> CMPB_R {
        CMPB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare C Default Output Value"]
    #[inline(always)]
    pub fn cmpc(&self) -> CMPC_R {
        CMPC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare D Default Output Value"]
    #[inline(always)]
    pub fn cmpd(&self) -> CMPD_R {
        CMPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare A Output Enable"]
    #[inline(always)]
    pub fn cmpaen(&self) -> CMPAEN_R {
        CMPAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare B Output Enable"]
    #[inline(always)]
    pub fn cmpben(&self) -> CMPBEN_R {
        CMPBEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare C Output Enable"]
    #[inline(always)]
    pub fn cmpcen(&self) -> CMPCEN_R {
        CMPCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare D Output Enable"]
    #[inline(always)]
    pub fn cmpden(&self) -> CMPDEN_R {
        CMPDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare A Default Output Value"]
    #[inline(always)]
    pub fn cmpa(&mut self) -> CMPA_W<0> {
        CMPA_W::new(self)
    }
    #[doc = "Bit 1 - Compare B Default Output Value"]
    #[inline(always)]
    pub fn cmpb(&mut self) -> CMPB_W<1> {
        CMPB_W::new(self)
    }
    #[doc = "Bit 2 - Compare C Default Output Value"]
    #[inline(always)]
    pub fn cmpc(&mut self) -> CMPC_W<2> {
        CMPC_W::new(self)
    }
    #[doc = "Bit 3 - Compare D Default Output Value"]
    #[inline(always)]
    pub fn cmpd(&mut self) -> CMPD_W<3> {
        CMPD_W::new(self)
    }
    #[doc = "Bit 4 - Compare A Output Enable"]
    #[inline(always)]
    pub fn cmpaen(&mut self) -> CMPAEN_W<4> {
        CMPAEN_W::new(self)
    }
    #[doc = "Bit 5 - Compare B Output Enable"]
    #[inline(always)]
    pub fn cmpben(&mut self) -> CMPBEN_W<5> {
        CMPBEN_W::new(self)
    }
    #[doc = "Bit 6 - Compare C Output Enable"]
    #[inline(always)]
    pub fn cmpcen(&mut self) -> CMPCEN_W<6> {
        CMPCEN_W::new(self)
    }
    #[doc = "Bit 7 - Compare D Output Enable"]
    #[inline(always)]
    pub fn cmpden(&mut self) -> CMPDEN_W<7> {
        CMPDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD0 Configuration\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0cfg](index.html) module"]
pub struct TCD0CFG_SPEC;
impl crate::attiny412pac::RegisterSpec for TCD0CFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcd0cfg::R](R) reader structure"]
impl crate::attiny412pac::Readable for TCD0CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd0cfg::W](W) writer structure"]
impl crate::attiny412pac::Writable for TCD0CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCD0CFG to value 0"]
impl crate::attiny412pac::Resettable for TCD0CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
