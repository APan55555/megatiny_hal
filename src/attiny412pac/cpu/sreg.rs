#[doc = "Register `SREG` reader"]
pub struct R(crate::attiny412pac::R<SREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREG` writer"]
pub struct W(crate::attiny412pac::W<SREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SREG_SPEC>;
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
impl From<crate::attiny412pac::W<SREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C` reader - Carry Flag"]
pub type C_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `C` writer - Carry Flag"]
pub type C_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `Z` reader - Zero Flag"]
pub type Z_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `Z` writer - Zero Flag"]
pub type Z_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `N` reader - Negative Flag"]
pub type N_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `N` writer - Negative Flag"]
pub type N_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `V` reader - Two's Complement Overflow Flag"]
pub type V_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `V` writer - Two's Complement Overflow Flag"]
pub type V_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `S` reader - N Exclusive Or V Flag"]
pub type S_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `S` writer - N Exclusive Or V Flag"]
pub type S_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `H` reader - Half Carry Flag"]
pub type H_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `H` writer - Half Carry Flag"]
pub type H_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `T` reader - Bit Copy Storage"]
pub type T_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `T` writer - Bit Copy Storage"]
pub type T_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
#[doc = "Field `I` reader - Global Interrupt Enable"]
pub type I_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `I` writer - Global Interrupt Enable"]
pub type I_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SREG_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Carry Flag"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Zero Flag"]
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Negative Flag"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two's Complement Overflow Flag"]
    #[inline(always)]
    pub fn v(&self) -> V_R {
        V_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - N Exclusive Or V Flag"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half Carry Flag"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit Copy Storage"]
    #[inline(always)]
    pub fn t(&self) -> T_R {
        T_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Carry Flag"]
    #[inline(always)]
    pub fn c(&mut self) -> C_W<0> {
        C_W::new(self)
    }
    #[doc = "Bit 1 - Zero Flag"]
    #[inline(always)]
    pub fn z(&mut self) -> Z_W<1> {
        Z_W::new(self)
    }
    #[doc = "Bit 2 - Negative Flag"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W<2> {
        N_W::new(self)
    }
    #[doc = "Bit 3 - Two's Complement Overflow Flag"]
    #[inline(always)]
    pub fn v(&mut self) -> V_W<3> {
        V_W::new(self)
    }
    #[doc = "Bit 4 - N Exclusive Or V Flag"]
    #[inline(always)]
    pub fn s(&mut self) -> S_W<4> {
        S_W::new(self)
    }
    #[doc = "Bit 5 - Half Carry Flag"]
    #[inline(always)]
    pub fn h(&mut self) -> H_W<5> {
        H_W::new(self)
    }
    #[doc = "Bit 6 - Bit Copy Storage"]
    #[inline(always)]
    pub fn t(&mut self) -> T_W<6> {
        T_W::new(self)
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline(always)]
    pub fn i(&mut self) -> I_W<7> {
        I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreg](index.html) module"]
pub struct SREG_SPEC;
impl crate::attiny412pac::RegisterSpec for SREG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sreg::R](R) reader structure"]
impl crate::attiny412pac::Readable for SREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sreg::W](W) writer structure"]
impl crate::attiny412pac::Writable for SREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREG to value 0"]
impl crate::attiny412pac::Resettable for SREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
