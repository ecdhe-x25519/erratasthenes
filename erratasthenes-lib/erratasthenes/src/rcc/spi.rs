use super::rcc::ClockEnable;
use stm32_metapac as pac;
use crate::spi::spi::*;

macro_rules! impl_spi_clock {
    ($name:ident, $en_bit:ident, $rst_bit:ident, $en_reg:ident, $rst_reg:ident) => {
        impl ClockEnable for $name {
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
    }
}

impl_spi_clock!(Spi1, set_spi1en, set_spi1rst, apb2enr, apb2rstr);
impl_spi_clock!(Spi2, set_spi2en, set_spi2rst, apb1enr1, apb1rstr1);