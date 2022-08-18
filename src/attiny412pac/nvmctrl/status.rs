#[doc = "Register `STATUS` reader"]
pub struct R(crate::attiny412pac::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FBUSY` reader - Flash busy"]
pub type FBUSY_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `EEBUSY` reader - EEPROM busy"]
pub type EEBUSY_R = crate::attiny412pac::BitReader<bool>;
#[doc = "Field `WRERROR` reader - Write error"]
pub type WRERROR_R = crate::attiny412pac::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Flash busy"]
    #[inline(always)]
    pub fn fbusy(&self) -> FBUSY_R {
        FBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EEPROM busy"]
    #[inline(always)]
    pub fn eebusy(&self) -> EEBUSY_R {
        EEBUSY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write error"]
    #[inline(always)]
    pub fn wrerror(&self) -> WRERROR_R {
        WRERROR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::attiny412pac::RegisterSpec for STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::attiny412pac::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::attiny412pac::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
