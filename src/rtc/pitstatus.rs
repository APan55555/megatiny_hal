#[doc = "Register `PITSTATUS` reader"]
pub struct R(crate::R<PITSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PITSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PITSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PITSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRLBUSY` reader - CTRLA Synchronization Busy Flag"]
pub type CTRLBUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTRLA Synchronization Busy Flag"]
    #[inline(always)]
    pub fn ctrlbusy(&self) -> CTRLBUSY_R {
        CTRLBUSY_R::new((self.bits & 1) != 0)
    }
}
#[doc = "PIT Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pitstatus](index.html) module"]
pub struct PITSTATUS_SPEC;
impl crate::RegisterSpec for PITSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pitstatus::R](R) reader structure"]
impl crate::Readable for PITSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PITSTATUS to value 0"]
impl crate::Resettable for PITSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
