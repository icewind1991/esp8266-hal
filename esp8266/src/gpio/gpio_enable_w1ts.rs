#[doc = "Register `GPIO_ENABLE_W1TS` writer"]
pub type W = crate::W<GPIO_ENABLE_W1TS_SPEC>;
#[doc = "Field `GPIO_ENABLE_DATA_W1TS` writer - Writing 1 into a bit in this register will set the related bit in GPIO_ENABLE_DATA"]
pub type GPIO_ENABLE_DATA_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_ENABLE_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:15 - Writing 1 into a bit in this register will set the related bit in GPIO_ENABLE_DATA"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_enable_data_w1ts(&mut self) -> GPIO_ENABLE_DATA_W1TS_W<GPIO_ENABLE_W1TS_SPEC> {
        GPIO_ENABLE_DATA_W1TS_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO_ENABLE_W1TS\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_enable_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_ENABLE_W1TS_SPEC;
impl crate::RegisterSpec for GPIO_ENABLE_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_enable_w1ts::W`](W) writer structure"]
impl crate::Writable for GPIO_ENABLE_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_ENABLE_W1TS to value 0"]
impl crate::Resettable for GPIO_ENABLE_W1TS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
