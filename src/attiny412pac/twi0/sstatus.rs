#[doc = "Register `SSTATUS` reader"]
pub struct R(crate::attiny412pac::R<SSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<SSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<SSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<SSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSTATUS` writer"]
pub struct W(crate::attiny412pac::W<SSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::attiny412pac::W<SSTATUS_SPEC>;
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
impl From<crate::attiny412pac::W<SSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::attiny412pac::W<SSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Slave Address or Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AP_A {
    #[doc = "0: Stop condition generated APIF"]
    STOP = 0,
    #[doc = "1: Address detection generated APIF"]
    ADR = 1,
}
impl From<AP_A> for bool {
    #[inline(always)]
    fn from(variant: AP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AP` reader - Slave Address or Stop"]
pub type AP_R = crate::attiny412pac::BitReader<AP_A>;
impl AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_A {
        match self.bits {
            false => AP_A::STOP,
            true => AP_A::ADR,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == AP_A::STOP
    }
    #[doc = "Checks if the value of the field is `ADR`"]
    #[inline(always)]
    pub fn is_adr(&self) -> bool {
        *self == AP_A::ADR
    }
}
#[doc = "Field `DIR` reader - Read/Write Direction"]
pub type DIR_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `BUSERR` reader - Bus Error"]
pub type BUSERR_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `BUSERR` writer - Bus Error"]
pub type BUSERR_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SSTATUS_SPEC, bool, O>;
#[doc = "Field `COLL` reader - Collision"]
pub type COLL_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `COLL` writer - Collision"]
pub type COLL_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SSTATUS_SPEC, bool, O>;
#[doc = "Field `RXACK` reader - Received Acknowledge"]
pub type RXACK_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `CLKHOLD` reader - Clock Hold"]
pub type CLKHOLD_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `APIF` reader - Address/Stop Interrupt Flag"]
pub type APIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `APIF` writer - Address/Stop Interrupt Flag"]
pub type APIF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SSTATUS_SPEC, bool, O>;
#[doc = "Field `DIF` reader - Data Interrupt Flag"]
pub type DIF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `DIF` writer - Data Interrupt Flag"]
pub type DIF_W<'a, const O: u8> = crate::attiny412pac::BitWriter<'a, u8, SSTATUS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Slave Address or Stop"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read/Write Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&self) -> BUSERR_R {
        BUSERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Collision"]
    #[inline(always)]
    pub fn coll(&self) -> COLL_R {
        COLL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Received Acknowledge"]
    #[inline(always)]
    pub fn rxack(&self) -> RXACK_R {
        RXACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Hold"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Flag"]
    #[inline(always)]
    pub fn apif(&self) -> APIF_R {
        APIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data Interrupt Flag"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn buserr(&mut self) -> BUSERR_W<2> {
        BUSERR_W::new(self)
    }
    #[doc = "Bit 3 - Collision"]
    #[inline(always)]
    pub fn coll(&mut self) -> COLL_W<3> {
        COLL_W::new(self)
    }
    #[doc = "Bit 6 - Address/Stop Interrupt Flag"]
    #[inline(always)]
    pub fn apif(&mut self) -> APIF_W<6> {
        APIF_W::new(self)
    }
    #[doc = "Bit 7 - Data Interrupt Flag"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W<7> {
        DIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Status\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read), [`write_with_zero`](crate::attiny412pac::generic::Reg::write_with_zero), [`reset`](crate::attiny412pac::generic::Reg::reset), [`write`](crate::attiny412pac::generic::Reg::write), [`modify`](crate::attiny412pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sstatus](index.html) module"]
pub struct SSTATUS_SPEC;
impl crate::attiny412pac::RegisterSpec for SSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sstatus::R](R) reader structure"]
impl crate::attiny412pac::Readable for SSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sstatus::W](W) writer structure"]
impl crate::attiny412pac::Writable for SSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SSTATUS to value 0"]
impl crate::attiny412pac::Resettable for SSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
