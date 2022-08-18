#[doc = "Register `OSC16ERR5V` reader"]
pub struct R(crate::attiny412pac::R<OSC16ERR5V_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::attiny412pac::R<OSC16ERR5V_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::attiny412pac::R<OSC16ERR5V_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::attiny412pac::R<OSC16ERR5V_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "OSC16 error at 5V\n\nThis register you can [`read`](crate::attiny412pac::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc16err5v](index.html) module"]
pub struct OSC16ERR5V_SPEC;
impl crate::attiny412pac::RegisterSpec for OSC16ERR5V_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [osc16err5v::R](R) reader structure"]
impl crate::attiny412pac::Readable for OSC16ERR5V_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OSC16ERR5V to value 0"]
impl crate::attiny412pac::Resettable for OSC16ERR5V_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
