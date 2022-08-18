#[doc = "Register `RXDATAH` reader"]
pub struct R(crate::attiny412pac::R<RXDATAH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<RXDATAH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<RXDATAH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<RXDATAH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA8` reader - Receiver Data Register"]
pub type DATA8_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PERR_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `FERR` reader - Frame Error"]
pub type FERR_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `BUFOVF` reader - Buffer Overflow"]
pub type BUFOVF_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `RXCIF` reader - Receive Complete Interrupt Flag"]
pub type RXCIF_R = crate::attiny412pac::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Data Register"]
    #[inline(always)]
    pub fn data8(&self) -> DATA8_R {
        DATA8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Frame Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer Overflow"]
    #[inline(always)]
    pub fn bufovf(&self) -> BUFOVF_R {
        BUFOVF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Complete Interrupt Flag"]
    #[inline(always)]
    pub fn rxcif(&self) -> RXCIF_R {
        RXCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Receive Data High Byte\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdatah](index.html) module"]
pub struct RXDATAH_SPEC;
impl crate::attiny412pac::RegisterSpec for RXDATAH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxdatah::R](R) reader structure"]
impl crate::attiny412pac::Readable for RXDATAH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAH to value 0"]
impl crate::attiny412pac::Resettable for RXDATAH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
