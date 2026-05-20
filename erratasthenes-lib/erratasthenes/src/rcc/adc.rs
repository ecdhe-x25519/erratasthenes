use super::rcc::ClockEnable;
use stm32_metapac as pac;
use crate::adc::adc::*;

macro_rules! impl_adc_clock {
    (
        $adc:ident,
        $en_bit:ident,
        $rst_bit:ident,
        $en_reg:ident,
        $rst_reg:ident
    ) => {
        impl ClockEnable for $adc {
            fn enable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$en_reg().modify(|w| w.$en_bit(true)); }
            }
            fn disable() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$en_reg().modify(|w| w.$en_bit(false)); }
            }
            fn reset() {
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$rst_reg().modify(|w| w.$rst_bit(true)); }
                core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
                unsafe { pac::rcc::Rcc::from_ptr(pac::RCC.as_ptr()).$rst_reg().modify(|w| w.$rst_bit(false)); }
            }
        }
    };
}

impl_adc_clock!(Adc1, set_adc12en, set_adc12rst, ahb2enr1, ahb2rstr1);
impl_adc_clock!(Adc2, set_adc12en, set_adc12rst, ahb2enr1, ahb2rstr1);

impl_adc_clock!(Adc4, set_adc4en, set_adc4rst, ahb3enr, ahb3rstr);